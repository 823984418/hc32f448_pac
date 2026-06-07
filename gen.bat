
del src\lib.rs
del build.rs
del device.x

svd2rust --edition 2024 --impl-debug --atomics --target cortex-m -i svd\HC32F448_patch.svd

form -i lib.rs -o src

del lib.rs

cargo fmt

cargo build
