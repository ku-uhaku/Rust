#!/bin/sh
set -eu

if [ "$#" -lt 1 ]; then
    printf 'Usage: %s <project-folder> [program arguments...]\n' "$0" >&2
    exit 2
fi

project_dir=$1
shift

if [ ! -d "$project_dir" ]; then
    printf 'Error: project folder not found: %s\n' "$project_dir" >&2
    exit 1
fi

if [ ! -f "$project_dir/Cargo.toml" ]; then
    printf 'Error: no Cargo.toml found in: %s\n' "$project_dir" >&2
    exit 1
fi

cd "$project_dir"
exec cargo run -- "$@"
