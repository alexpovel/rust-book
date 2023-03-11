projects := `find . -type f -name 'Cargo.toml' -printf '%h\n'`

# Lists all available commands.
default:
    @just --list

# Runs a cargo command on all found Cargo projects.
sweep cmd:
    @echo "{{projects}}" | xargs -L 1 sh -c 'cd "$0" && cargo {{cmd}}'

# Builds a Cargo project in the given directory.
build dir: (cargo 'build' dir)

# Tests a Cargo project in the given directory.
test dir: (cargo 'test' dir)

# Runs a given Cargo command in the given directory.
cargo cmd dir:
    cd {{justfile_directory() / dir}} && cargo {{cmd}}
