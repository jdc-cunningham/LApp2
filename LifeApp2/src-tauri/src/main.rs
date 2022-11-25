#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// https://stackoverflow.com/questions/19605132/is-it-possible-to-use-global-variables-in-rust

use std::io::Write;
use rusqlite::{Connection, Result};
// use chrono::{DateTime, Utc}

#[derive(Debug)]
// struct Note {
// 	id: i32,
// 	name: String,
// 	body: String,
// 	created_at: DateTime<Utc>,
// 	updated_at: DateTime<Utc>,
// }

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
