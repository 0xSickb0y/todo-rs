# todo-rs

A simple and efficient TODO application written in Rust.

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/0xSickb0y/todo-rs.git
   cd todo-rs
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Install the binary:**
   ```bash
   cargo install --path .
   ```

## Configuration

The application stores its data in your system's config directory:

- **Linux/macOS**: `~/.config/todo-rs/` or `$XDG_CONFIG_HOME/todo-rs/`
- **Database file**: `tasks.db` within the config directory

The application will automatically create the necessary directories and database file on first run.

## Usage

```bash
# Add a new task:
$ todo-rs add "Buy groceries"
$ todo-rs add "Finish the project proposal"

#List all tasks:
$ todo-rs list

# Mark a task as complete:
$ todo-rs done <ID>

# Remove a task:
$ todo-rs remove <ID>
```

### Examples

```bash
# Add some tasks
$ todo-rs add "Write documentation"
Task added successfully with id: 1

$ todo-rs add "Review pull requests"
Task added successfully with id: 2

# List all tasks
$ todo-rs list
ID       | DONE     | BIRTH               | DESCRIPTION
------------------------------------------------------------
1        | false    | 2024-12-07 14:30:15 | Write documentation
2        | false    | 2024-12-07 14:30:22 | Review pull requests

# Mark a task as done
$ todo-rs done 1
Task 1 marked as done!

# List tasks again
$ todo-rs list
ID       | DONE     | BIRTH               | DESCRIPTION
------------------------------------------------------------
1        | true     | 2024-12-07 14:30:15 | Write documentation
2        | false    | 2024-12-07 14:30:22 | Review pull requests

# Remove a task
$ todo-rs remove 2
Task 2 removed!
```

## Development

Prerequisites:

- Rust 1.70 or later
- Cargo (comes with Rust)


### Documentation

Generate and view the documentation:

```bash
$ cargo doc --open
```


## Architecture

The application is structured into several modules:

- **`main.rs`** - Entry point and application orchestration
- **`args.rs`** - Command line argument parsing using clap
- **`config.rs`** - Configuration directory management with XDG compliance
- **`database.rs`** - Database operations and SQL query management
- **`models.rs`** - Data models and database interaction methods
- **`io_utils.rs`** - Input/output utility functions


## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.