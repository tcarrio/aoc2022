# aoc2022

Advent of Code 2022: Rusty edition

## Project Structure

The repository utilizes **cargo workspaces** in order to reuse certain functionality across challenges.

You can find all of the challenges under the [challenges](./challenges) folder.

Miscellanious crates can be found under the [crates](./crates) folder.

You can execute a specific package and binary with cargo easily with:

```shell
cargo run -p day2 --bin part1
```

Test input is typically saved in a file in that challenge folder (ie. `./challenges/day2/input.txt`).

So the overall execution of the test input for day 2 part 1 would be done with:

```shell
cat ./challenges/day2/input.txt | cargo run -p day2 --bin part1
```

## Completion

I'll fill out a table for the challenge completion eventually
