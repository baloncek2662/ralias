# ralias

A command line interface tool for managing your aliases.

Run using

```bash
cargo run
```

Build a release version and copy the binary to a directory in your executable
path, `/usr/bin` for example:

```bash
cargo build --release
cp ./target/release/ralias /usr/bin
```

Add a wrapper around the tool in your interactive terminal init script
(eg. `~/.bashrc`) so that you do not need to reload init the script
every time you modify an alias:

```
ralias()
{
    /usr/bin/ralias $@
    source ~/.bashrc
}
```
