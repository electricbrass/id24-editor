# ID24 JSON Editor

A simple (and extremely unfinished) GUI for editing ID24 JSON lumps.

# Build Instructions

On Windows and Mac, all dependencies are handled by Cargo.
Just run `cargo build --release`

Linux requires `cosmic-icons`. See below on how to install for your distro:

Pop!_OS 24.04 or later:\
Preinstalled

Fedora 41 or later:\
`dnf install cosmic-icon-theme`

Everything else:\
Install [just](https://github.com/casey/just)
```sh
git clone https://github.com/pop-os/cosmic-icons
cd cosmic-icons
sudo just install
```
