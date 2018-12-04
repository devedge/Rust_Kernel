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

  - `qemu-system-x86_64 -drive format=raw,file=target/x86_64-rustkernelv2/debug/bootimage-rustkernelv2.bin`, or `bootimage run`

    Run the kernel in QEMU

  - `qemu-system-x86_64 -drive format=raw,file=target/x86_64-rustkernelv2/debug/bootimage-rustkernelv2.bin -serial mon:stdio`, or `bootimage run -- -serial mon:stdio`

    Run kernel in QEMU with serial arguments

  - `bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04`

    Run kernel in QEMU with serial arguments and shutdown functionality

  - `qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none`

    or:

    `bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none`

    Run as above, but hide the graphical QEMU window

  - `bootimage run --bin test-template -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none`

    Run the binary test `test-template`
