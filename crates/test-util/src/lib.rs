use rand::{distributions, thread_rng, Rng};
use std::{env, fs, io, path::PathBuf};
use tempfile::{tempdir, TempDir};

pub enum FixtureType {
    File(String),
    Content(String),
}

pub struct FileFixture {
    pub path: PathBuf,
    // source: PathBuf,
    _tempdir: TempDir,
}

impl FileFixture {
    pub fn load(&self) -> io::Result<Vec<u8>> {
        fs::read(&self.path)
    }

    pub fn from(fixture_type: FixtureType) -> FileFixture {
        match fixture_type {
            FixtureType::File(filepath) => FileFixture::from_file(&filepath),
            FixtureType::Content(content) => FileFixture::from_content(&content),
        }
    }

    fn from_file(fixture_filename: &str) -> FileFixture {
        // First, figure out the right file in `tests/fixtures/`:
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root_dir);
        source.push("tests/fixtures");
        source.push(&fixture_filename);

        // The "real" path of the file is going to be under a temporary directory:
        let tempdir = match tempdir() {
            Ok(dir) => dir,
            Err(_) => panic!("Failed to open temporary directory"),
        };
        let mut path = PathBuf::from(&tempdir.path());
        path.push(&fixture_filename);

        if fs::copy(&source, &path).is_err() {
            panic!("Failed to copy from source to path");
        }

        FileFixture {
            _tempdir: tempdir,
            // source,
            path,
        }
    }

    fn from_content(fixture_content: &str) -> FileFixture {
        // let source = PathBuf::new();

        let fixture_filename = get_random_string(32);

        // The "real" path of the file is going to be under a temporary directory:
        let tempdir = match tempdir() {
            Ok(dir) => dir,
            Err(_) => panic!("Failed to open temporary directory"),
        };
        let mut path = PathBuf::from(&tempdir.path());
        path.push(&fixture_filename);

        if fs::write(&path, fixture_content).is_err() {
            panic!("Failed to write content to path: {}", path.to_str().unwrap());
        }

        FileFixture {
            _tempdir: tempdir,
            // source,
            path,
        }
    }
}

pub fn fail() {
    panic!("A test failure has been forced!");
}

fn get_random_string(len: usize) -> String {
    thread_rng()
        .sample_iter::<u8, _>(distributions::Alphanumeric)
        .map(|byte| byte as char)
        .take(len)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_fixture() {
        let expected_content = "A\nB\nC\n".as_bytes();

        let file = FileFixture::from(FixtureType::File(String::from("test.txt")));
        let maybe_content = fs::read(file.path);

        match maybe_content {
            Ok(actual_content) => assert_eq!(expected_content, actual_content),
            Err(_) => fail(),
        }
    }

    #[test]
    #[should_panic = r#"Failed to copy from source to path"#]
    fn test_nonexistent_file_fixture() {
        FileFixture::from(FixtureType::File(String::from("i-should-not-exist-test.txt")));
    }

    #[test]
    fn test_file_fixture_with_content() {
        let expected_content = "X\nY\nZ\n".as_bytes();

        let file = FileFixture::from(FixtureType::Content(String::from("X\nY\nZ\n")));
        let maybe_content = fs::read(file.path);

        match maybe_content {
            Ok(actual_content) => assert_eq!(expected_content, actual_content),
            Err(_) => fail(),
        }
    }
}
