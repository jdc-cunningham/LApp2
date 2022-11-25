// fails need sqlite crate

fn main() {
  let search_term: &str = "t";
	let connection = sqlite::open("file.db").unwrap();
	let query = "SELECT name FROM notes WHERE name LIKE ?";

	let wildcard_start = "%".to_string();
	let wildcard_end = "%".to_string();
	let wcd_search_term = String::from(wildcard_start + "t" + &wildcard_end);

	for row in connection
    .prepare(query)
    .unwrap()
    .into_iter()
    .bind((1, wcd_search_term))
    .unwrap()
    .map(|row| row.unwrap())
	{
		// (row.read::<&str, _>("name"));
			// println!("body = {}", row.read::<&str, _>("body"));
	}
}