# bullet
The Bullet Linux distribution's repository

## Components
- Bullet Repository              ~ build scripts for packages in the Stable and Lab channel
- Bullet Templates               ~ templates for Bullet packages. Also contains `pkmake.sh`, an utility for creating new packages.
- ABS (Automated Build System)   ~ program for building packages and images for Bullet Linux
- bpkg                           ~ Bullet Linux's package manager

## Possible future Projects (all those projects are independent from bullet, and they will be )
- AIS (Automated Install System) ~ Easy but flexbile system for automaticaly installing Bullet Linux on different machines.
- coffee-init                    ~ Fuck systemd.
- nic (Nic is not CUPS)          ~ Fuck CUPS.
- reblued                        ~ (Reliable bluetooth daemon). Fuck BlueZ.
- neoland                        ~ Wayland compositor. Fully customizable and re-programmable in Lisp.

## Concepts

- Small collection of packages = packages are more curated and tested
- Packages get updated in Lab as soon as possible (rolling release design).
- Bulk updates (ex. if lib-x and program-y get a new update, and program-y requires the new version of lib-x, we will always make the new versions avaiable at the same time, so that it isn't possible to partially update the system, ).
- When an library gets updated, all packages that depends on it will be recompiled against the new version of the library. (This brings more stability)
- ATO (Automated Testing Oriented) -> Packages can implement a `test` command, which can be useful with unit testing. This will than be used in order to test if packages are well working before going to the stable or in some cases before going to Lab.
- A simple, flexible, compatible, reproducable and recoverable package manager:
    - Simple: `install`, `remove`, `clean`, `update`. Is this easy enough?
    - Flexible:
    - Compatible: you can download binaries from Bullet Archives, compile official packages from the Bullet Repository, compile community-provided packages from the Community Bullet Repository, you can directly install `.deb` files or install them, with their dependencies (if they aren't found in the Bullet Repository) from any deb-compatible remote mirror (ex. Debian, Ubuntu, etc...).
    - Reproducable: Bpkg allows you to export the entire or part of the package tree (with the version of the packages), import it in another system in order to have an exact copy, embed it in packages in order to have the advantages of Docker (without being a container), and even building ISO and OCI images.
    - Recoverable: Bpkg takes ispiration from git; in fact,

## How to install the local Toolchain

1) Install the Rust compiler.
2) `cd bpkg && cargo build --release && cd ..`
3) `cd abs && cargo build --release && cd ..`
4) `./install.sh`

## How to build the rootfs

For Stable releases:
- Build Rootfs: `abs rootfs repos/stable repos/stable.list rootfs.tar`
For Lab releases:
- Build Rootfs: `abs rootfs repos/lab repos/lab.list rootfs.tar`

## How to build images

After that, you can build:
- ISO image: `abs iso rootfs.tar bullet.iso`
- OCI image: `abs oci rootfs.tar bullet.oci`
