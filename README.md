# joycon

Easy way to switch `hosts` / `kube/config` (maybe something else) by a single command.

Do not provide released packages for different platforms yet.

```bash
sudo joycon
```

## Usage

Clone this repository first ->

Install Rust for your local environment, and run the build command with release tag.

```bash
cargo build --release
```

This command should generate a dir named `target`, you can directly execute the file `target/release/joycon`.

If you wanna do this as a command, copy the file to `usr/local/bin`

## License

[MIT](./LICENSE) License © 2024-PRESENT [Tamago](https://github.com/tmg0)
