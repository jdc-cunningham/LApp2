#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// https://stackoverflow.com/questions/19605132/is-it-possible-to-use-global-variables-in-rust
// https://diesel.rs/guides/getting-started.html
// https://stackoverflow.com/questions/58296263/how-do-i-create-a-new-database-using-diesel

use std::io::Write;
// use rusqlite::{Connection, Result};
// use chrono::{DateTime, Utc}
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::envy;

pub fn establish_connection() -> SqliteConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	SqliteConnection::establish(&database_url)
		.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// https://stackoverflow.com/a/41688369/2710227
// https://stackoverflow.com/a/27588417/2710227
fn log(msg: &str) {
	std::io::stderr().write(format!("{}\n", msg).as_bytes()).ok(); // msg
}

fn start_db() {
	let connection = Connection::open("file.db");

	let query = "
		DROP TABLE notes;
		DROP TABLE tags;
		CREATE TABLE notes (
			id INTEGER primary key autoincrement,
			name TEXT,
			body TEXT,
			tags TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
		INSERT INTO notes (name, body, tags) VALUES ('test', 'test', '[]');
		CREATE TABLE tags (
			id INTEGER primary key autoincrement,
			name TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	";

	connection
		.execute(
			query,
			(),
		);
}

// concatenate string, jeez why is this so hard
// https://stackoverflow.com/a/30154791/2710227
// https://maxuuell.com/blog/how-to-concatenate-strings-in-rust
#[tauri::command]
fn search(search_term: &str) {
	let connection = Connection::open("file.db");
	let query = "SELECT name FROM notes WHERE name LIKE ?";

	let wildcard_start = "%".to_string();
	let wildcard_end = "%".to_string();
	let wcd_search_term = String::from(wildcard_start + search_term + &wildcard_end);


}

fn main() {
	start_db();

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![search])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
