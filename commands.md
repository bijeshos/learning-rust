
# Install

- To install `rustup` (on unix based machines)
    - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    
    - Above command will download and install the official compiler for the Rust
programming language, and its package manager, `Cargo`.

    - Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

        - /home/<user_dir>/.rustup

    - The Cargo home directory located at:
        - /home/<user_dir>/.cargo

    - The `cargo`, `rustc`, `rustup` and other commands will be added to
Cargo's bin directory, located at:
        - /home/<user_dir>/.cargo/bin

    - This path will be added to PATH environment variable by
modifying the profile files located at:
        - /home/<user_dir>/.profile
        - /home/<user_dir>/.bashrc

# Uninstall

- On unix based machines
    - $ `rustup self uninstall`


# To compile
- $ r`ustc <file-name>.rs`

# To create a new project with Cargo
- $ `cargo new project_name`
    - This will create a project which contains the following:
        - Cargo.toml
            - configuration file in `Tom’s Obvious, Minimal Language` format   
        - src/main.rs
            - a sample test file

# To build
- $ `cargo build`
    - creates a binary in `target/debug` directory with the project's name
    - Run `./target/debug/<project_name>` to execute

# To compile code and run
- $ `cargo run`

# To check
- $ `cargo check`
    - This checks whether the code is compilable or not 
    - This is much faster than `cargo build`

# Build for release
- $ `cargo build --release`