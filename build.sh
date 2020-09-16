if [[ $1 == "debug" ]]; then 
    build=""
else 
    build="--release"
fi

echo "Clean target"
cargo clean

if [[ $? -ne 0 ]]; then
    echo
    >&2 echo "clean command failed"
    exit 1
fi

echo "Clippy $build"
echo
cargo clippy $build -- -D clippy::all -D clippy::pedantic -D clippy::nursery -A clippy::missing-errors-doc -A clippy::must-use-candidate

if [[ $? -ne 0 ]]; then
    echo
    >&2 echo "clippy command failed"
    exit 2
fi

echo
echo "Tests $build"
echo
cargo test --all $build

if [[ $? -ne 0 ]]; then
    echo
    >&2 echo "build command failed"
    exit 3
fi

echo
echo "Build $build"
echo
cargo build $build

if [[ $? -ne 0 ]]; then
    echo
    >&2 echo "run command failed"
    exit 4
fi

if [[ $2 == "run" ]]; then
    echo
    echo "Run $build"
    echo
    cargo run $build
    echo
fi