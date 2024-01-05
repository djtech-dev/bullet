# ABS - Automated Build System

## Features for end-users

- Building multiple packages from a Repository
- Building rootfs of Bullet Linux from built packages
- Building ISO & OCI images from rootfs

## Features for mantainers/CI

- Updating the package source code and re-building using cached artifacts
- Re-building package when dependencies change
- Managing the release of bulk updates
- AutoTesting packages
- Checking if there are possible dependency collisions

## `build_lab.sh`

`build_lab.sh` is the ABS's script that prepares the Lab releases. This is normally executed by Github CI.
In order, it:

- Downloads the source codes of the updated packages
- Builds it & packages it
- Tests the package, checks globally for dependency collisions and makes a report that will be viewed by the mantainers/develoeprs. If the package fails either the test or the dependency collision check, it will be flagged as PSB (Possible System Breaker).
- (ONLY IF THIS PACKAGE IS A LIBRARY) Rebuilds all packages that dependens on this package & repackage them. (If the original packages is flagged as PSB, those dependencies will also be flagged as PSB).
- Makes the packages available to the repository.
