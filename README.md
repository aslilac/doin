# doin

### Installation

```bash
cargo install doin
```

I'll probably come up with some way to install prebuilt binaries later, so you don't have
to have Rust/Cargo installed.

### Usage

```
➜  pwd
/Projects/example/
➜  doin ./src/ pwd
/Projects/example/src/
➜  doin ./src/ mkdir ./foo/
➜  doin ./src/foo/ pwd
/Projects/example/src/foo/
```

Run commands easily in another directory! Useful for bash scripting, or if you frequently
need to run commands in a bunch of different directories, but usually only run them one
at a time. You don't need to cd in and out constantly, just point at each one as needed.

First argument is the directory to run the command in, and the second is the command to run.
Any arguments after that will be passed to the command as-is, in order.
