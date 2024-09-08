# `enm` - Encrypted Note Manager

Enm is an encrypted note manager that uses `PBKDF2` with `ChaCha20Poly1305` to encrypt arbitrary notes on your filesystem. Enm was created as an exercise to familiarise myself with the `egui` library. That being said security was still a top priority while developing this application. To ensure a secure environment, we utilize the `zeroize` crate to nullify the memory that contains secret data post-encryption.

Enm is available for **Linux**, **Mac OS** and **Windows**.

## Install

Installation can either be done via building from source or through the pre-built binaries.

### Download pre-built binaries



### Build from source
Ensure you have the latest version of Rust installed.

1. Clone the repository
```sh
git clone https://github.com/starkbamse/enm
```

2. Change directory
```sh
cd enm
```

3. Build the project
```sh
cargo build --release
```

4. Execute
```sh
target/release/enm
```

