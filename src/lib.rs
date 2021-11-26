use time::{Date, OffsetDateTime};

pub struct Arvelie {
	year: i32,
	month: char,
	day: u8,
}

impl Arvelie {
	pub fn from_utc(epoch: Date) -> Self {
		Self::from_date(epoch, OffsetDateTime::now_utc().date())
	}

	pub fn from_date(epoch: Date, date: Date) -> Self {
		let arvelie_year = date.year() - epoch.year();
		let ordinal_zero_based = date.ordinal() - 1;

		if date.ordinal() == 365 {
			Self {
				year: arvelie_year,
				month: '+',
				day: 0,
			}
		} else if date.ordinal() == 366 {
			Self {
				year: arvelie_year,
				month: '+',
				day: 1,
			}
		} else {
			let arvelie_month = ((ordinal_zero_based / 14) as u8 + 'A' as u8) as char;
			let arvelie_day = (ordinal_zero_based % 14) as u8;

			Self {
				year: arvelie_year,
				month: arvelie_month,
				day: arvelie_day,
			}
		}
	}

	pub fn as_string(&self) -> String {
		format!("{:02}{}{:02}", self.year, self.month, self.day)
	}

	pub fn year(&self) -> i32 {
		self.year
	}

	pub fn month(&self) -> char {
		self.month
	}

	pub fn month_digit(&self) -> Option<u8> {
		if self.month == '+' {
			None
		} else {
			Some(self.month as u8 - 'A' as u8)
		}
	}

	pub fn day(&self) -> u8 {
		self.day
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use time::Date;

	fn check(year: i32, month: (char, Option<u8>), day: u8, arv: Arvelie) {
		assert_eq!(year, arv.year());
		assert_eq!(month.0, arv.month());
		assert_eq!(month.1, arv.month_digit());
		assert_eq!(day, arv.day());

		let expected_str = format!("{:02}{}{:02}", year, month.0, day);
		assert_eq!(expected_str, arv.as_string())
	}

	#[test]
	fn test_zero_based_feb18() {
		check(
			1,
			('D', Some(3)),
			6,
			Arvelie::from_date(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2007, 02, 18).unwrap(),
			),
		);
	}

	#[test]
	fn test_zero_based_jan1() {
		check(
			2,
			('A', Some(0)),
			0,
			Arvelie::from_date(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2008, 01, 01).unwrap(),
			),
		);
	}

	#[test]
	fn january_29th() {
		check(
			4,
			('C', Some(2)),
			0,
			Arvelie::from_date(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2010, 01, 29).unwrap(),
			),
		);
	}

	#[test]
	fn december_31st_no_leap() {
		check(
			4,
			('+', None),
			0,
			Arvelie::from_date(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2010, 12, 31).unwrap(),
			),
		);
	}

	#[test]
	fn december_31st_with_leap() {
		check(
			2,
			('+', None),
			1,
			Arvelie::from_date(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2008, 12, 31).unwrap(),
			),
		);
	}
}
