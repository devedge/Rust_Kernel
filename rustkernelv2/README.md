- Commands used to build the kernel
  - `cargo rustc -- -Z pre-link-arg=-nostartfiles`
    Build freestanding binary on Linux
  - `cargo build`
    Build freestanding binary on Windows
  - `cargo rustc -- -Z pre-link-arg=-lSystem`
    Build freestanding binary on macOS
  - `cargo xbuild --target x86_64-rustkernelv2.json`
    Build kernel against custom target
