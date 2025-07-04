# todo-rs

A simple and efficient TODO application written in Rust.

## Features

- Add, edit, and delete tasks
- Mark tasks as complete or incomplete
- List all tasks with their status
- Command-line interface for ease of use
- Fast and minimal dependencies

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

3. **Run the application:**
   ```bash
   cargo run
   ```

## Usage

- Add a new task:
  ```
  todo-rs add "<TASK_DESCRIPTION>"
  ```

- List all tasks:
  ```
  todo-rs list
  ```

- Mark a task as complete:
  ```
  todo-rs done <ID>
  ```

- Delete a task:
  ```
  todo-rs delete <ID>
  ```

## Contributing

Contributions are welcome! Please open issues or submit pull requests for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
