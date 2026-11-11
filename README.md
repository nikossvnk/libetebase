<h1 align="center">Etebase - Encrypt Everything</h1>

> [!NOTE]
> **This is an independent fork** of [libetebase](https://github.com/etesync/libetebase), the original
> project of the EteSync authors. This fork is not affiliated with, endorsed by or supported by EteSync or
> Etebase in any way. The history of the original project is kept as it is, with its authors, and the
> changes of this fork follow it. Like the original, it's licensed under the [BSD-3-Clause](./LICENSE) license.

A C library for Etebase

This package is implemented in Rust and exposes a C API for people to use.

![GitHub tag](https://img.shields.io/github/tag/nikossvnk/libetebase.svg)

# Documentation

The C API is described in the header `etebase.h`, which `make` generates (with cbindgen) next to the library.

# Build

```
make
```

# Install

```
sudo make install
```
