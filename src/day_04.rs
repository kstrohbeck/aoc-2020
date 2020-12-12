use self::{color::Color, eye_color::EyeColor, height::Height, year::Year};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, value},
    sequence::{pair, preceded, tuple},
    Finish, IResult,
};
use std::str::FromStr;

pub fn star_1(data: String) {}

pub fn star_2(data: String) {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Passport {
    birth_year: Year,
    issue_year: Year,
    expiration_year: Year,
    height: Height,
    hair_color: Color,
    eye_color: EyeColor,
    passport_id: u32,
    country_id: Option<String>,
}

mod year {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Year(pub u32);

    impl FromStr for Year {
        type Err = ();

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            all_consuming(year)(input)
                .finish()
                .map_err(|_| ())
                .map(|(_, x)| x)
        }
    }

    fn year(input: &str) -> IResult<&str, Year> {
        map_res(
            take_while_m_n(4, 4, |c: char| c.is_ascii_digit()),
            |x: &str| x.parse::<u32>().map(Year),
        )(input)
    }

    #[cfg(test)]
    mod tests {
        use super::{year, Year};

        #[test]
        fn from_str_requires_all_input_to_be_consumed() {
            assert_eq!("2020...".parse::<Year>(), Err(()));
        }

        #[test]
        fn year_parses() {
            assert_eq!(year("2020..."), Ok(("...", Year(2020))));
        }
    }
}

mod height {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Height {
        pub amount: u32,
        pub measurement: Measurement,
    }

    impl Height {
        fn new(amount: u32, measurement: Measurement) -> Self {
            Self {
                amount,
                measurement,
            }
        }
    }

    impl FromStr for Height {
        type Err = ();

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            all_consuming(height)(input)
                .finish()
                .map_err(|_| ())
                .map(|(_, x)| x)
        }
    }

    fn height(i: &str) -> IResult<&str, Height> {
        fn u32_(i: &str) -> IResult<&str, u32> {
            map_res(digit1, |s: &str| s.parse::<u32>())(i)
        }

        map(pair(u32_, measurement), |(amt, msr)| Height::new(amt, msr))(i)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Measurement {
        Inches,
        Centimeters,
    }

    fn measurement(input: &str) -> IResult<&str, Measurement> {
        alt((
            value(Measurement::Inches, tag("in")),
            value(Measurement::Centimeters, tag("cm")),
        ))(input)
    }

    #[cfg(test)]
    mod tests {
        use super::{height, measurement, Height, Measurement};

        #[test]
        fn from_str_requires_all_input_to_be_consumed() {
            assert_eq!("123cm ".parse::<Height>(), Err(()));
        }

        #[test]
        fn height_parses_inches() {
            assert_eq!(
                height("123in..."),
                Ok((
                    "...",
                    Height {
                        amount: 123,
                        measurement: Measurement::Inches
                    }
                ))
            );
        }

        #[test]
        fn height_parses_centimeters() {
            assert_eq!(
                height("123cm..."),
                Ok((
                    "...",
                    Height {
                        amount: 123,
                        measurement: Measurement::Centimeters
                    }
                ))
            );
        }

        #[test]
        fn measurement_parses_inches() {
            assert_eq!(measurement("in..."), Ok(("...", Measurement::Inches)));
        }

        #[test]
        fn measurement_parses_centimeters() {
            assert_eq!(measurement("cm..."), Ok(("...", Measurement::Centimeters)));
        }
    }
}

mod color {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Color {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    impl Color {
        fn new(red: u8, green: u8, blue: u8) -> Self {
            Self { red, green, blue }
        }
    }

    impl FromStr for Color {
        type Err = ();

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            all_consuming(color)(input)
                .finish()
                .map_err(|_| ())
                .map(|(_, x)| x)
        }
    }

    fn hex_byte(input: &str) -> IResult<&str, u8> {
        map_res(take_while_m_n(2, 2, |c: char| c.is_digit(16)), |i| {
            u8::from_str_radix(i, 16)
        })(input)
    }

    fn color(input: &str) -> IResult<&str, Color> {
        map(
            preceded(tag("#"), tuple((hex_byte, hex_byte, hex_byte))),
            |(r, g, b)| Color::new(r, g, b),
        )(input)
    }

    #[cfg(test)]
    mod tests {
        use super::{color, Color};

        #[test]
        fn from_str_requires_all_input_to_be_consumed() {
            assert_eq!("#a0c1e2...".parse::<Color>(), Err(()));
        }

        #[test]
        fn color_parses() {
            assert_eq!(
                color("#a0c1e2..."),
                Ok((
                    "...",
                    Color {
                        red: 0xa0,
                        green: 0xc1,
                        blue: 0xe2
                    }
                ))
            );
        }
    }
}

mod eye_color {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EyeColor {
        Amber,
        Blue,
        Brown,
        Gray,
        Green,
        Hazel,
        Other,
    }

    impl FromStr for EyeColor {
        type Err = ();

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            all_consuming(eye_color)(input)
                .finish()
                .map_err(|_| ())
                .map(|(_, x)| x)
        }
    }

    fn eye_color(input: &str) -> IResult<&str, EyeColor> {
        alt((
            value(EyeColor::Amber, tag("amb")),
            value(EyeColor::Blue, tag("blu")),
            value(EyeColor::Brown, tag("brn")),
            value(EyeColor::Gray, tag("gry")),
            value(EyeColor::Green, tag("grn")),
            value(EyeColor::Hazel, tag("hzl")),
            value(EyeColor::Other, tag("oth")),
        ))(input)
    }

    #[cfg(test)]
    mod tests {
        use super::{eye_color, EyeColor};

        #[test]
        fn from_str_requires_all_input_to_be_consumed() {
            assert_eq!("amb...".parse::<EyeColor>(), Err(()));
        }

        #[test]
        fn eye_color_parses() {
            assert_eq!(eye_color("amb..."), Ok(("...", EyeColor::Amber)));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PassportEntry<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

/*
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while_m_n},
    character::complete::{digit1, multispace0, space0, line_ending},
    combinator::{map_opt, map_res, not, opt, value},
    multi::many1,
    sequence::{preceded, separated_pair, tuple}
};
use std::str::FromStr;

pub fn star_1(data: String) {
    let passports = parse(&data);
    let count = passports.iter().filter(|p| p.is_valid_npc()).count();
    println!("{}", count);
}

pub fn star_2(data: String) {
    let passports = parse(&data);
    let count = passports.iter().filter(|p| p.is_really_valid_npc()).count();
    println!("{}", count);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Passport<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn is_valid_npc(&self) -> bool {
        [
            self.birth_year,
            self.issue_year,
            self.expiration_year,
            self.height,
            self.hair_color,
            self.eye_color,
            self.passport_id,
        ]
            .iter()
            .all(Option::is_some)
    }

    fn birth_year(&self) -> Option<u32> {
        let byr = self.birth_year?;
        if byr.len() != 4 {
            return None;
        }
        let byr = byr.parse::<u32>().ok()?;
        if byr < 1920 || byr > 2002 {
            return None;
        }
        Some(byr)
    }

    fn is_birth_year_valid(&self) -> bool {
        self.birth_year().is_some()
    }

    fn issue_year(&self) -> Option<u32> {
        let iyr = self.issue_year?;
        if iyr.len() != 4 {
            return None;
        }
        let iyr = iyr.parse::<u32>().ok()?;
        if iyr < 2010 || iyr > 2020 {
            return None;
        }
        Some(iyr)
    }

    fn is_issue_year_valid(&self) -> bool {
        self.issue_year().is_some()
    }

    fn expiration_year(&self) -> Option<u32> {
        let eyr = self.expiration_year?;
        if eyr.len() != 4 {
            return None;
        }
        let eyr = eyr.parse::<u32>().ok()?;
        if eyr < 2020 || eyr > 2030 {
            return None;
        }
        Some(eyr)
    }

    fn is_expiration_year_valid(&self) -> bool {
        self.expiration_year().is_some()
    }

    fn height(&self) -> Option<(u32, Measurement)> {
        let (_, hgt) = height(self.height?).ok()?;
        let (lo, hi) = match hgt.1 {
            Measurement::Inches => (59, 76),
            Measurement::Centimeters => (150, 193),
        };
        if hgt.0 < lo || hgt.0 > hi {
            return None;
        }
        Some(hgt)
    }

    fn is_height_valid(&self) -> bool {
        self.height().is_some()
    }

    fn eye_color(&self) -> Option<EyeColor> {
        self.eye_color?.parse().ok()
    }

    fn is_eye_color_valid(&self) -> bool {
        self.eye_color().is_some()
    }

    fn hair_color(&self) -> Option<Color> {
        let (_, color) = hex_color(self.hair_color?).ok()?;
        Some(color)
    }

    fn is_hair_color_valid(&self) -> bool {
        self.hair_color().is_some()
    }

    fn is_passport_id_valid(&self) -> bool {
        if let Some(pid) = self.passport_id {
            pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
        } else {
            false
        }
    }

    fn is_really_valid_npc(&self) -> bool {
        self.is_birth_year_valid()
            && self.is_issue_year_valid()
            && self.is_expiration_year_valid()
            && self.is_height_valid()
            && self.is_eye_color_valid()
            && self.is_hair_color_valid()
            && self.is_passport_id_valid()
        /*
        if !self.is_birth_year_valid() {
            return false;
        }

        if !self.is_issue_year_valid() {
            return false;
        }

        if !self.is_expiration_year_valid() {
            return false;
        }

        if !self.is_height_valid() {
            return false;
        }

        if !self.is_eye_color_valid() {
            return false;
        }

        if !self.is_hair_color_valid() {
            return false;
        }

        if !self.is_passport_id_valid() {
            return false;
        }

        true
        */
    }
}

impl<'a> Default for Passport<'a> {
    fn default() -> Self {
        Self {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }
}

fn parse<'a>(i: &'a str) -> Vec<Passport<'a>> {
    passport_list(i).unwrap().1
}

fn passport_list<'a>(i: &'a str) -> IResult<&'a str, Vec<Passport<'a>>> {
    many1(passport)(i)
}

fn passport<'a>(i: &'a str) -> IResult<&'a str, Passport<'a>> {
    let (i, _) = multispace0(i)?;
    let (i, fields) = many1(passport_field)(i)?;
    let mut passport = Passport::default();
    for (key, value) in fields {
        match key {
            "byr" => passport.birth_year = Some(value),
            "iyr" => passport.issue_year = Some(value),
            "eyr" => passport.expiration_year = Some(value),
            "hgt" => passport.height = Some(value),
            "hcl" => passport.hair_color = Some(value),
            "ecl" => passport.eye_color = Some(value),
            "pid" => passport.passport_id = Some(value),
            "cid" => passport.country_id = Some(value),
            _ => {}
        }
    }
    Ok((i, passport))
}

fn passport_field<'a>(i: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    preceded(
        tuple((space0, opt(line_ending), space0, not(line_ending))),
        separated_pair(
            take_until(":"),
            tag(":"),
            take_till(|c: char| c.is_ascii_whitespace()),
        ),
    )(i)
}


fn hex_byte(i: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, |c: char| c.is_digit(16)),
        |i| u8::from_str_radix(i, 16),
    )(i)
}

fn hex_color(i: &str) -> IResult<&str, Color> {
    let (i, _) = tag("#")(i)?;
    let (i, red) = hex_byte(i)?;
    let (i, green) = hex_byte(i)?;
    let (i, blue) = hex_byte(i)?;
    Ok((i, Color { red, green, blue }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Gray),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazel),
            "oth" => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Passport, passport, passport_list};

    #[test]
    fn full_passport_parses() {
        let text = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n";
        let (i, pass) = passport(text).unwrap();
        assert_eq!(pass, Passport {
            birth_year: Some("1937"),
            issue_year: Some("2017"),
            expiration_year: Some("2020"),
            height: Some("183cm"),
            hair_color: Some("#fffffd"),
            eye_color: Some("gry"),
            passport_id: Some("860033327"),
            country_id: Some("147"),
        });
        assert_eq!(i, "\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n");
    }

    #[test]
    fn passport_list_parses() {
        let text = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n";
        let (i, passports) = passport_list(text).unwrap();
        assert_eq!(passports[0], Passport {
            birth_year: Some("1937"),
            issue_year: Some("2017"),
            expiration_year: Some("2020"),
            height: Some("183cm"),
            hair_color: Some("#fffffd"),
            eye_color: Some("gry"),
            passport_id: Some("860033327"),
            country_id: Some("147"),
        });
        assert_eq!(passports[1], Passport {
            birth_year: Some("1929"),
            issue_year: Some("2013"),
            expiration_year: Some("2023"),
            height: None,
            hair_color: Some("#cfa07d"),
            eye_color: Some("amb"),
            passport_id: Some("028048884"),
            country_id: Some("350"),

        })
    }
}

*/
