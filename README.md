# acli

**acli** is a terminal-based AI agent built with Rust.

This project is primarily a **learning project**. I'm building it to learn Rust by actually building something with it, while also exploring how AI CLI agents work under the hood.

Gemini is currently being used as the AI provider, but the project is intentionally structured so that supporting other providers can be explored later.

> ⚠️ **Status:** Experimental / actively under development.
> This project is being built for learning and experimentation rather than production use.

## Why?

I've always been interested in how tools like AI coding agents and terminal assistants actually work.

Instead of only using them, I wanted to build one myself.

`acli` is my way of learning:

* Rust
* Async programming
* HTTP APIs
* JSON serialization and deserialization
* CLI application development
* AI API integration
* How AI agents are structured internally

The goal isn't to build the perfect AI agent. The goal is to understand **how these systems are built by building one myself**.

## Getting Started

### Prerequisites

You'll need:

* [Rust](https://www.rust-lang.org/tools/install)
* A Gemini API key

You can check that Rust is installed with:

```bash
rustc --version
cargo --version
```

### Clone the repository

```bash
git clone <repository-url>
cd acli
```

### Configure the API key

Create a `.env` file in the root of the project:

```env
GEMINI_API_KEY=your_api_key_here
```

Make sure `.env` is included in `.gitignore` so your API key isn't committed to the repository.

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

Once running, `acli` can be interacted with directly from the terminal.

To exit the program:

```text
exit
```

## Development

Since this is a learning project, I primarily run it directly through Cargo while developing:

```bash
cargo run
```

For checking the project without running it:

```bash
cargo check
```

And for building a release version:

```bash
cargo build --release
```

## Project Structure

```text
acli/
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
└── src/
    ├── config.rs
    ├── extract_content.rs
    ├── gemini.rs
    ├── input.rs
    ├── lib.rs
    ├── main.rs
    └── models.rs
```

The project is intentionally being kept relatively small while I learn how the different pieces fit together.

## Current Technology

The project currently uses:

* **Rust** — application language
* **Tokio** — asynchronous runtime
* **Reqwest** — HTTP requests
* **Serde / Serde JSON** — JSON serialization and deserialization
* **dotenvy** — environment variable management
* **Gemini API** — current AI provider

## Learning Goals

The main goal of `acli` is not simply to create another AI CLI.

I'm using the project to learn Rust concepts through a real application, including:

* Ownership and borrowing
* Structs and enums
* Modules and project organization
* Error handling with `Result` and `Option`
* Async/await
* HTTP requests
* Working with external APIs
* JSON parsing
* Environment variables
* CLI input/output
* Building applications with Cargo

At the same time, I'm learning what actually goes into an AI agent rather than treating the model API as a black box.

## Disclaimer

`acli` is a personal learning project and is **not intended to be production-ready**.

The code, architecture, and APIs may change significantly as I learn more Rust and experiment with different approaches to building AI agents.

## License

See [LICENSE](LICENSE) for the project's license.
