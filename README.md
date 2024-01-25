
# Tugas.rs

`Tugas.rs` is a simple, command-line to-do list application written in Rust. It provides a straightforward way to manage your tasks directly from your terminal. The application allows you to add, list, complete, check, and delete tasks efficiently.

## Features

- **Add Tasks**: Quickly add tasks to your to-do list.
- **List Tasks**: View all your current tasks.
- **Complete Tasks**: Mark tasks as completed.
- **Check Tasks**: Check specific tasks in your list.
- **Delete Tasks**: Remove tasks from your list, either individually or all at once.

## Installation

To use `Tugas.rs`, you need to have Rust installed on your machine. If you don't have Rust installed, you can download it from [the official Rust website](https://www.rust-lang.org/).

### Building from Source

Clone the repository:

```bash
git clone https://github.com/ahmad-alkadri/tugas.rs.git
cd tugas.rs
```

Build the project using Cargo:

```bash
cargo build --release
```

The built binary will be located at `target/release/tugas`.

## Usage

After building the application, you can run it directly from the command line. Here are some examples of how you can use `Tugas.rs`:

### Adding a Task

```bash
./tugas add "Your task description here"
```

### Listing Tasks

```bash
./tugas list
```

### Completing a Task

```bash
./tugas complete 1
```

### Checking Specific Tasks

```bash
./tugas check 1,2,3
```

### Deleting Tasks

Delete specific tasks:

```bash
./tugas delete 1,3
```

Delete all tasks:

```bash
./tugas delete all
```

## Contributing

Contributions to `Tugas.rs` are welcome! Please feel free to submit pull requests, report bugs, or suggest new features.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
