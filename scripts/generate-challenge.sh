#!/usr/bin/env sh

set -e

PROJECT_ROOT="$( cd "$( dirname "$0" )" && cd .. && pwd )"

source "$PROJECT_ROOT/scripts/assert.sh"

assert_truthy "$PROJECT_ROOT"

TEMPLATE_DIR="$PROJECT_ROOT/templates/challenge"

main() {
    local day_number="$1"

    assert_gt "$day_number" 0 "You must provide a day number"

    local target_dir="$PROJECT_ROOT/challenges/day${day_number}"

    if [ -d "$target_dir" ]; then
        echo "The challenge for day $day_number already exists"
        exit 1
    fi

    cp -r "$TEMPLATE_DIR" "$target_dir"

    find "$target_dir" -type f -name '*.tpl' | while read f; do
        sed -i '.old' "s/##DAY##/${day_number}/g" "$f"
        mv "$f" "${f%.tpl}"
        rm "$f.old"
    done
}

main "$@"