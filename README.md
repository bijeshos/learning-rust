# Learning Rust
Programs and materials used for Rust learning and training.

# Repo Structure
All examples are available inside the `src` directory. This project is created as both binary crate and library crate. That means, both `main.rs` and `lib.rs` files are present. 
`main.ts` file acts as an entry point to the program, in case you want to execute all the examples. `lib.rs` acts as the wrapper program to the examples. 

For ease of reference, source code for each area is grouped under separate packages. Each package has a wrapper function ( in the format `run_*_examples`), which acts as an entry point to the examples of that particular package. 
Each example is wrapped inside a separate function that can be executed independently (except in a few cases, which are explicitly mentioned in the code )

# Examples covered

At the moment, following topics/examples are covered:

|Topic|Package|File|Examples|
|--|--|--|--|
|Variable declaration|`variables`|`variables.rs`|<ul><li>Immutable</li><li>Mutable</li><li>Constant</li><li>Shadowing</li></ul> |
|Scalar Data Types|`data_types`|`data_types.rs`|<ul><li>Unsigned Integer</li><li>Signed Integer</li><li>Float</li><li>Boolean</li><li>Character</li></ul>|
|Compound Data Types|`data_types`|`data_types.rs`|<ul><li>Tuple</li><li>Array</li></ul>|
|Functions|`functions`|`functions.rs`|<ul><li>Function without args and return value</li><li>Public function</li><li>Private function</li><li>Function with one argument and single return value</li><li>Function with multiple arguments and multiple return values</li></ul>|
|Control Flows|`controle_flows`|`controle_flows.rs`|<ul><li>if-else</li></ul>|

Following topics/examples would be added soon:

|Topic|Package|File|Examples|
|--|--|--|--|
|Memory Management|-|-|-|
|Slices|-|-|-|
|Enums|-|-|-|
|Error Handling|-|-|-|
|Generics|-|-|-|
|I/O|-|-|-|
|Smart Pointers|-|-|-|
|Concurrency|-|-|-|
|Unsafe|-|-|-|
|Automated Tests|-|-|-|
 

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

- To uninstall `rustup` (on unix based machines)
    - $ `rustup self uninstall`


# To compile
- $ `rustc <file-name>.rs`

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
  - To run without any arguments
- $ `cargo run arg1 arg2`
  -  - To run without with arguments (e.g.: `arg1` and `arg2`)

# To check
- $ `cargo check`
    - This checks whether the code is compilable or not 
    - This is much faster than `cargo build`

# Build for release
- $ `cargo build --release`

# Comment style
- The comments in the code is usually kept in separate line above the line it's annotating (exceptions would be mentioned explicitly)

# *work in progress*
- Note: This is a work-in-progress repo. More examples would be added soon. 