pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_insensitive: bool
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
	let contents = std::fs::read_to_string(config.filename)?;

	if config.case_insensitive {
		for line in search_case_insensitive(&config.query, &contents) {
			println!("{}", line);
		}
	}
	else {
		for line in search(&config.query, &contents) {
			println!("{}", line);
		}
	}

	Ok(())
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}

		let mut case_insensitive = false;
		for el in args {
			if el == "-c"
			{
				case_insensitive = true;
			}
		}

		let query = args[1].clone();
		let filename = args[2].clone();

		Ok(Config {query, filename, case_insensitive})
	}
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

	results
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "safe";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
	}
}