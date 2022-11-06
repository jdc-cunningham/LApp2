#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// https://docs.rs/sqlite/latest/sqlite/
// https://stackoverflow.com/questions/19605132/is-it-possible-to-use-global-variables-in-rust
// static mut DB: Option<sqlite::Connection> = None;

use std::io::Write;

// https://stackoverflow.com/a/41688369/2710227
// https://stackoverflow.com/a/27588417/2710227
fn log(msg: &str) {
	std::io::stderr().write(format!("{}\n", msg).as_bytes()).ok(); // msg
}

// unwrap https://stackoverflow.com/questions/21257686/what-is-this-unwrap-thing-sometimes-its-unwrap-sometimes-its-unwrap-or
fn start_db() {
	// unsafe {
		// DB = Some(sqlite::open(":memory:").unwrap()); // unwrap ensures ok value
	// }

	let DB = sqlite::open(":memory:").unwrap();

	DB
    .execute(
			"CREATE TABLE notes (
				id INTEGER primary key autoincrement,
				name TEXT,
				body TEXT,
				tags TEXT,
				created_at TIMESTAMP CURRENT_TIMESTAMP,
				updated_at TIMESTAMP CURRENT_TIMESTAMP
			);",
    )
    .unwrap();
}

#[tauri::command]
fn search(search_term: &str) {
	log(search_term);
}

fn main() {
	start_db();

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![search])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
