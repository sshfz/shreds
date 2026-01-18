# Rust Redis Clone

A simple Redis-like key-value store implemented in Rust for learning and experimentation.

## Prerequisites

* Rust (stable) installed
* Cargo (comes with Rust)

Check installation:

```bash
rustc --version
cargo --version
```

## How to Run

Clone the repository and navigate into the project directory:

```bash
git clone <your-repo-url>
cd <your-project-folder>
```

Run the project using Cargo:

```bash
cargo run
```

This will:

* Compile the project
* Start the Redis-like server

By default, the server listens on `127.0.0.1:6379` (or whatever port you configured).

## Testing with redis-cli

If you have Redis installed, you can test using:

```bash
redis-cli -p 6379
```

Example commands:

```text
SET name Alice
GET name
```

JSON values can be stored as strings:

```text
SET user "{\"id\":1,\"name\":\"Alice\"}"
GET user
```

## Notes

* This project mimics **core Redis behavior**
* Values are stored as raw strings/bytes
* No JSON validation is performed (same as real Redis)

## Learning Goals

* Understand TCP servers in Rust
* Learn RESP protocol parsing
* Practice safe concurrency with `Arc`, `RwLock`, and async Rust

---
