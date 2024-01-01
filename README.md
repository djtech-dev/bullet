# bullet
The Bullet Linux distribution's repository

## Components
- Bullet Repository            ~ build scripts for packages in the Stable and Lab channel
- ABS (Automated Build System) ~ program for building packages and images for Bullet Linux
- bpkg                         ~ Bullet Linux's package manager

## How to build

1) Install the Rust compiler.
2) `cd bpkg && cargo build --release && cd ..`
3) `ln -s bpkg/target/release/bpkg bpkg`
4) `cd abs && cargo build --release && cd ..`
5) `ln -s abs/target/release/abs abs`

For Stable releases:
- Build Rootfs: `./abs rootfs repos/stable repos/stable.list rootfs.tar`
For Lab releases:
- Build Rootfs: `./abs rootfs repos/lab repos/lab.list rootfs.tar`

After that, you can build:
- ISO image: `./abs iso rootfs.tar bullet.iso`
- OCI image: `./abs oci rootfs.tar bullet.oci`
