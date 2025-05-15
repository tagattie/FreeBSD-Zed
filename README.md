# Zed port for FreeBSD

---

This repository contains an experimental port of Zed for FreeBSD.

## Get
A package file is available at the releases page. To install, download a file and run the command:

``` shell
pkg install zed-editor-<version>.pkg
```

## Run

``` shell
zedit
```

## Build
If you would like to build artifacts for yourself, be sure you have the ports tree on your machine. Clone this repository with git command and build/install Zed:

``` shell
git clone https://github.com/tagattie/FreeBSD-Zed
cd FreeBSD-Zed/editors/zed
make install clean
```
