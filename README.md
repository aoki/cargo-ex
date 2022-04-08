# cargo-ex

This cargo sub-command execute `cargo run --example` easily via fuzzy finder.

<video style="border-radius: 6px;" src="https://user-images.githubusercontent.com/1206676/162441748-c917fc08-a0c3-4fd6-bdb4-49adfcfce0ed.mp4" width="100%"></video>

## Install

`cargo install cargo-ex`

## Usage

There are some files in the `examples` directory.

```shellsession
$ ls -1 examples
hello-cargo.rs
hello-example.rs
hello-rust.rs
hello-wave.rs
```

It shows files using a fuzzy finder in the `examples` directory when you execute `cargo ex`.

```shellsession
$ cargo ex
  hello-rust.rs
  hello-cargo.rs
> hello-example.rs
  hello-wave.rs
  4/4                                                                       1/0
> heo
```

Then you press the enter to execute `cargo run --examples`.

```shellsession
$ cargo ex
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/hello-example`
Hello example!
```
