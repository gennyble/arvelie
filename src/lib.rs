use time::Date;

pub fn to_arvelie_string(epoch: Date, date: Date) -> String {
	let arvelie_year = date.year() - epoch.year();
	let ordinal_zero_based = date.ordinal() - 1;

	if date.ordinal() == 365 {
		format!("{:02}+00", arvelie_year)
	} else if date.ordinal() == 366 {
		format!("{:02}+01", arvelie_year)
	} else {
		let arvelie_month = ((ordinal_zero_based / 14) as u8 + 'A' as u8) as char;
		let arvelie_day = ordinal_zero_based % 14;

		format!("{:02}{}{:02}", arvelie_year, arvelie_month, arvelie_day)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use time::Date;

	#[test]
	fn test_zero_based_feb18() {
		assert_eq!(
			"01D06",
			to_arvelie_string(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2007, 02, 18).unwrap()
			)
		);
	}

	#[test]
	fn test_zero_based_jan1() {
		assert_eq!(
			"02A00",
			to_arvelie_string(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2008, 01, 01).unwrap()
			)
		);
	}

	#[test]
	fn january_29th() {
		assert_eq!(
			"04C00",
			to_arvelie_string(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2010, 01, 29).unwrap()
			)
		);
	}

	#[test]
	fn december_31st_no_leap() {
		assert_eq!(
			"04+00",
			to_arvelie_string(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2010, 12, 31).unwrap()
			)
		);
	}

	#[test]
	fn december_31st_with_leap() {
		assert_eq!(
			"02+01",
			to_arvelie_string(
				Date::try_from_yo(2006, 1).unwrap(),
				Date::try_from_ymd(2008, 12, 31).unwrap()
			)
		);
	}
}
