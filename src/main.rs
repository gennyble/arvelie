use arvelie::Arvelie;
use confindent::Confindent;
use dirs::config_dir;
use time::Date;

fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();

	match args.len() {
		0 => {
			let mut config_path = config_dir().unwrap();
			config_path.push("arvelie.conf");

			let conf = match Confindent::from_file(&config_path) {
				Ok(cnfi) => cnfi,
				Err(e) => {
					eprintln!(
						"Couldn't read config at '{}': {}",
						config_path.to_string_lossy(),
						e
					);
					return;
				}
			};

			let epoch: i32 = match conf.child_parse("EpochYear") {
				Ok(e) => e,
				Err(e) => {
					eprintln!(
						"Couldn't parse EpochYear from '{}': {}",
						config_path.to_string_lossy(),
						e
					);
					return;
				}
			};

			println!(
				"{}",
				Arvelie::from_utc(Date::try_from_yo(epoch, 1).unwrap()).as_string()
			)
		}
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
			let mut config_path = config_dir().unwrap();
			config_path.push("arvelie.conf");

			println!("Usage: arvelie [epoch_year] [date]\nIf no epoch_year is provided, its read from {}\nIf no date is provided, the current date in UTC is returned.", config_path.to_string_lossy());
		}
	}
}
