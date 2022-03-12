Setup deviates from the book. See https://github.com/rust-in-action/code/issues/44.

```bash
rustup toolchain install nightly-2021-03-01
rustup default nightly-2021-03-01
```

If running from a "remote" IDE, Qemu will need to display on the VM's desktop.  Set the following:

```bash
 export DISPLAY=:1
```

To run, use the following command:

```bash
cargo +nightly-2021-03-01 run
```
