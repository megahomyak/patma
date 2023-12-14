use std::borrow::Borrow;

pub struct Pattern {
    regex: regex::Regex,
}

pub enum PatternPart {
    VerbatimString(String),
    AnyCharacter,
    AnySequence,
}

pub enum PatternCreationError {
    TwoAdjacentAnySequences,
}

impl Pattern {
    pub fn new(first_char: char, middle_parts: &[PatternPart], last_char: char) -> Result<Self, PatternCreationError> {
        let mut regex = String::new();
        regex.push_str(&regex::escape(first_char));
        for part in middle_parts {
            match part {
                PatternPart::VerbatimString(string) => regex.push_str()
            }
            regex.push_str(r"([^\(\);]*)");
            regex.push_str(&regex::escape(part));
        }
        regex.push_str(&regex::escape(last_char));
        let mut regex = String::new();
        regex.push_str(&regex::escape(first_part));
        Self {
            regex: regex::Regex::new(&regex).unwrap(),
        }
    }
}

pub struct Pattern {
    regex: regex::Regex,
}

pub enum SubstitutionPatternKind {
    OneCharacter,
    AnyAmountOfCharacters,
}

pub struct NonEmptyString {
    pub first_character: char,
    pub rest: String,
}

impl<T: Borrow<NonEmptyString>> From<T> for String {
    fn from(string: T) -> Self {
        format!("{}{}", string.first_character, string.rest)
    }
}

pub struct SubstitutionPattern {
    pub kind: SubstitutionPatternKind,
    pub next_substring: NonEmptyString,
}

impl Pattern {
    pub fn new(first_part: &NonEmptyString, rest: &[&SubstitutionPattern]) -> Self {
        let first_part: String = first_part.into();
        let mut regex = String::new();
        regex.push_str(&regex::escape(first_part));
        for part in rest {
            regex.push_str(r"([^\(\);]*)");
            regex.push_str(&regex::escape(part));
        }
        Self {
            regex: regex::Regex::new(&regex).unwrap(),
        }
    }
}

pub struct Matches<'a> {
    contents: Vec<&'a str>,
}

impl<'a> Matches<'a> {
    pub fn get(&self, index: usize) -> &str {
        self.contents.get(index).unwrap_or(&"")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Match {
    /// Inclusive
    pub beginning_byte_index: usize,
    /// Inclusive
    pub end_byte_index: usize,
    pub groups: Vec<String>,
}

impl Pattern {
    pub fn first_match<'a>(&self, string: &'a str) -> Option<Match> {
        self.regex.captures(string).map(|captures| {
            let mut captures_iter = captures.iter();
            captures_iter.next(); // Skipping the zero group
            let groups = captures_iter
                .map(|capture| {
                    capture
                        .map(|capture| capture.as_str())
                        .unwrap_or("")
                        .to_owned()
                })
                .collect();
            Match {
                beginning_byte_index: captures.get(0).unwrap().start(),
                end_byte_index: captures.get(0).unwrap().end(),
                groups,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups<const N: usize>(array: [&str; N]) -> Vec<String> {
        Vec::from(array.map(|s| s.to_owned()))
    }

    #[test]
    fn nothing_captured() {
        let pattern = Pattern::new("a", &["b", "c"]);
        assert_eq!(
            pattern.first_match("blahabcdef"),
            Some(Match {
                groups: groups(["", ""]),
                beginning_byte_index: 4,
                end_byte_index: 7
            })
        );
    }

    #[test]
    fn parenthesis() {
        let pattern = Pattern::new("a", &["b", "c"]);
        assert_eq!(
            pattern.first_match("(print ())"),
            Some(Match {
                groups: groups(["", ""]),
                beginning_byte_index: 4,
                end_byte_index: 7
            })
        );
    }
}
