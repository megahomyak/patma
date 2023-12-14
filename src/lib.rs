pub struct Pattern {
    regex: regex::Regex,
}

enum PatternPart {
    VerbatimChar(char),
    AnyCharacter,
    AnySequence,
}

pub enum PatternCreationError {
    AdjacentStars,
}

fn take_one_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    chars.next().map(|c| (c, chars.as_str()))
}

fn parse_one_char(s: &str) -> Option<(PatternPart, &str)> {
    take_one_char(s).map(|(c, rest)| match c {
        '*' => (PatternPart::AnySequence, rest),
        '.' => (PatternPart::AnyCharacter, rest),
        '\\' => match take_one_char(rest) {
            Some((next_c, next_rest)) => match next_c {
                '*' | '.' | '\\' => (PatternPart::VerbatimChar(next_c), next_rest),
                _ => (PatternPart::VerbatimChar(c), rest),
            },
            None => (PatternPart::VerbatimChar(c), rest),
        },
        _ => (PatternPart::VerbatimChar(c), rest),
    })
}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Self, PatternCreationError> {
        let mut regex = String::new();
        regex.push_str(&regex::escape(first_char));
        for part in middle_parts {
            match part {
                PatternPart::VerbatimString(string) => regex.push_str(),
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
