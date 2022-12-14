#!/usr/bin/env sh

set -e

PROJECT_ROOT="$( cd "$( dirname "$0" )" && cd .. && pwd )"

source "$PROJECT_ROOT/scripts/assert.sh"

assert_truthy "$PROJECT_ROOT"

TEMPLATE_DIR="$PROJECT_ROOT/templates/challenge"

mac_replace_templates() {
    local f="$1"
    local day_number="$2"

    sed -i '.old' "s/##DAY##/${day_number}/g" "$f"
    mv "$f" "${f%.template}"
    rm "$f.old"
}

linux_replace_templates() {
    local f="$1"
    local day_number="$2"

    sed -i "s/##DAY##/${day_number}/g" "$f"
    mv "$f" "${f%.template}"
}

replace_templates() {
    case "$(uname)" in
        Darwin)
            mac_replace_templates "$@";;
        *)
            linux_replace_templates "$@";;
    esac
}

main() {
    local day_number="$1"

    assert_gt "$day_number" 0 "You must provide a day number"

    local target_dir="$PROJECT_ROOT/challenges/day${day_number}"

    if [ -d "$target_dir" ]; then
        echo "The challenge for day $day_number already exists"
        exit 1
    fi

    cp -r "$TEMPLATE_DIR" "$target_dir"

    find "$target_dir" -type f -name '*.template' | while read f; do
        replace_templates "$f" "$day_number"
    done
}

main "$@"