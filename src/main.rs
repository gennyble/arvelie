use arvelie::Arvelie;
use time::Date;

fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();

	match args.len() {
		1 => {
			println!(
				"{}",
				Arvelie::from_utc(Date::try_from_yo(args[0].parse::<i32>().unwrap(), 1).unwrap())
					.as_string()
			);
		}
		2 => {
			println!(
				"{}",
				Arvelie::from_date(
					Date::try_from_yo(args[0].parse::<i32>().unwrap(), 1).unwrap(),
					Date::parse(&args[1], "%F").unwrap()
				)
				.as_string()
			);
		}
		_ => {
			println!("Usage: arvelie [epoch_year] [date]\nIf no date is provided, the current date in UTC is returned.");
		}
	}
}
