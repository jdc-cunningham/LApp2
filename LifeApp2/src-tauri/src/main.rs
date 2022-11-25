#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// https://docs.rs/sqlite/latest/sqlite/
// https://stackoverflow.com/questions/19605132/is-it-possible-to-use-global-variables-in-rust

use std::io::Write;

// https://stackoverflow.com/a/41688369/2710227
// https://stackoverflow.com/a/27588417/2710227
fn log(msg: &str) {
	std::io::stderr().write(format!("{}\n", msg).as_bytes()).ok(); // msg
}

// unwrap https://stackoverflow.com/questions/21257686/what-is-this-unwrap-thing-sometimes-its-unwrap-sometimes-its-unwrap-or
// https://stackoverflow.com/a/5097712/2710227
fn start_db() {
	// going to do it this way, easier for me
	// apparently better to close it
	// https://stackoverflow.com/a/62994222/2710227
	let connection = sqlite::open("file.db").unwrap();

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

	connection.execute(query).unwrap();
}

// concatenate string, jeez why is this so hard
// https://stackoverflow.com/a/30154791/2710227
// https://maxuuell.com/blog/how-to-concatenate-strings-in-rust
#[tauri::command]
fn search(search_term: &str) {
	let connection = sqlite::open("file.db").unwrap();
	let query = "SELECT name FROM notes WHERE name LIKE ?";

	let wildcard_start = "%".to_string();
	let wildcard_end = "%".to_string();
	let wcd_search_term = String::from(wildcard_start + search_term + &wildcard_end);

	// let rows = connection
  //   .prepare(query)
  //   .unwrap()
  //   .bind(1, search_term)
  //   .unwrap();

	// for row in connection
  //   .prepare(query)
  //   .unwrap()
  //   .into_iter()
  //   .bind((1, wcd_search_term))
  //   .unwrap()
  //   .map(|row| row.unwrap())
	// {
	// 	// (row.read::<&str, _>("name"));
	// 		// println!("body = {}", row.read::<&str, _>("body"));
	// }

	let mut statement = connection.prepare(query).unwrap();
	statement.bind((1, wcd_search_term)).unwrap();
	statement.into_iter().map(|row| row.unwrap());
}

fn main() {
	start_db();

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![search])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
