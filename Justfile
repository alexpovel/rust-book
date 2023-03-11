build-all: (build 'guessing_game') (build 'hello') (build 'hello_cargo') (build 'minigrep')

build dir:
    cd {{justfile_directory() / dir}} && cargo build --release
