use std::process::exit;

mod args;
mod config;
mod database;
mod io_utils;

fn main() {
    let command = args::parse_args();

    let home_directory = config::get_home_dir();
    config::create_config_path(&home_directory);

    let db_path = database::get_db_path(&home_directory);

    if !database::check_db_exists(&db_path) {
        println!("Database not found at {}", db_path.display());

        if io_utils::ask_user_confirmation("Do you want to create it? (Y/N): ") {
            database::create_database(&db_path);
            println!("Database created at {}", db_path.display());
        } else {
            println!("Goodbye!");
            exit(1);
        }
    }

    database::handle_db_operations(&db_path, command);
}
