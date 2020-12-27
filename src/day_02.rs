use crate::utils::u32_;
use nom::IResult;

pub fn star_1(data: String) {
    let entries = parse(&data);
    let valid_entries = entries.iter().filter(|e| e.is_valid()).count();
    println!("{}", valid_entries);
}

pub fn star_2(data: String) {
    let entries = parse(&data);
    let valid_entries = entries.iter().filter(|e| e.is_valid_2()).count();
    println!("{}", valid_entries);
}

fn parse<'a>(data: &'a str) -> Vec<PasswordEntry<'a>> {
    password_entries(data).unwrap().1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PasswordEntry<'a> {
    password: &'a str,
    rule_min: u32,
    rule_max: u32,
    rule_char: char,
}

impl<'a> PasswordEntry<'a> {
    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.rule_char)
            .count() as u32;
        count >= self.rule_min && count <= self.rule_max
    }

    fn is_valid_2(&self) -> bool {
        let count = self
            .password
            .chars()
            .enumerate()
            .filter(|(i, _)| *i + 1 == self.rule_min as usize || *i + 1 == self.rule_max as usize)
            .filter(|(_, c)| *c == self.rule_char)
            .count();

        count == 1
    }
}

fn password_entries<'a>(i: &'a str) -> IResult<&'a str, Vec<PasswordEntry<'a>>> {
    use nom::{character::complete::multispace1, multi::separated_list0};

    separated_list0(multispace1, password_entry)(i)
}

fn password_entry<'a>(i: &'a str) -> IResult<&'a str, PasswordEntry<'a>> {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, anychar, space1},
    };

    let (i, rule_min) = u32_(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, rule_max) = u32_(i)?;
    let (i, _) = space1(i)?;
    let (i, rule_char) = anychar(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = space1(i)?;
    let (i, password) = alpha1(i)?;
    let entry = PasswordEntry {
        password,
        rule_min,
        rule_max,
        rule_char,
    };
    Ok((i, entry))
}

#[cfg(test)]
mod tests {
    use super::{password_entries, password_entry, PasswordEntry};

    #[test]
    fn password_entry_parses() {
        let i = "1-3 a: abcde";
        let (_, entry) = password_entry(i).unwrap();
        assert_eq!(
            entry,
            PasswordEntry {
                password: "abcde",
                rule_min: 1,
                rule_max: 3,
                rule_char: 'a',
            }
        );
    }

    #[test]
    fn password_entries_parse() {
        let i = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n";
        let (_, entries) = password_entries(i).unwrap();
        assert_eq!(
            entries,
            vec![
                PasswordEntry {
                    password: "abcde",
                    rule_min: 1,
                    rule_max: 3,
                    rule_char: 'a',
                },
                PasswordEntry {
                    password: "cdefg",
                    rule_min: 1,
                    rule_max: 3,
                    rule_char: 'b',
                },
                PasswordEntry {
                    password: "ccccccccc",
                    rule_min: 2,
                    rule_max: 9,
                    rule_char: 'c',
                },
            ]
        );
    }
}
