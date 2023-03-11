test-all: (test 'guessing_game') (test 'hello') (test 'hello_cargo') (test 'minigrep')

build-all: (build 'guessing_game') (build 'hello') (build 'hello_cargo') (build 'minigrep')

build dir: (cargo 'build' dir)

test dir: (cargo 'test' dir)

cargo cmd dir:
    cd {{justfile_directory() / dir}} && cargo {{cmd}}
