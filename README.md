# fforget-bin

Simple command line utility that attempts to hint Linux to remove the files at
the provided paths from the page cache, and then (optionally, but enabled by
default) checks if they still reside in memory (only performing the check is
also possible).

Check the built-in `--help` menu for more information.

## Install

Using [cargo](https://rustup.rs/) (from GitHub):

```console
$ cargo install --git https://github.com/ckatsak/fforget-bin
```
