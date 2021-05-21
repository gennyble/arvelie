use arvelie;

use time::Date;
use time::OffsetDateTime;

fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();

	match args.len() {
		1 => {
			println!(
				"{}",
				arvelie::to_arvelie_string(
					Date::try_from_yo(args[0].parse::<i32>().unwrap(), 1).unwrap(),
					OffsetDateTime::now_utc().date()
				)
			);
		}
		2 => {
			println!(
				"{}",
				arvelie::to_arvelie_string(
					Date::try_from_yo(args[0].parse::<i32>().unwrap(), 1).unwrap(),
					Date::parse(&args[1], "%F").unwrap()
				)
			);
		}
		_ => {
			println!("Usage: arvelie [epoch_year] [date]\nIf no date is provided, the current date in UTC is returned.");
		}
	}
}
