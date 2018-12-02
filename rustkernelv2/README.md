- Commands used to build the kernel
  - `cargo rustc -- -Z pre-link-arg=-nostartfiles`
    Build freestanding binary on Linux
  - `cargo build`
    Build freestanding binary on Windows
  - `cargo rustc -- -Z pre-link-arg=-lSystem`
    Build freestanding binary on macOS
  - `cargo xbuild --target x86_64-rustkernelv2.json`
    Build freestanding binary against custom kernel target
  - `bootimage build`
    Build kernel using bootimage and options from Cargo.toml
  - `qemu-system-x86_64 -drive format=raw,file=target/x86_64-rustkernelv2/debug/bootimage-rustkernelv2.bin`
    Run the kernel in qemu
