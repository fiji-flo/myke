# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd)

    test -f Cargo.lock || cargo generate-lockfile

    cross build --target $TARGET --release

    cp target/$TARGET/release/myke $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET
}

main
