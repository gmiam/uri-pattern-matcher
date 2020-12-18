#[derive(Debug, Clone)]
pub(crate) enum PatternPart<'a> {
    Joker,
    Value(&'a str),
}

impl<'a> From<&'a str> for PatternPart<'a> {
    fn from(part: &'a str) -> Self {
        match part {
            p if p.starts_with('{') && p.ends_with('}') => Self::Joker,
            p => Self::Value(p),
        }
    }
}

impl<'a> From<&PatternPart<'a>> for usize {
    fn from(part: &PatternPart) -> Self {
        match part {
            PatternPart::Joker => 1,
            PatternPart::Value(_) => 0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usize_from_pattern_part() {
        let parts: Vec<PatternPart> = vec![
            PatternPart::Joker,
            PatternPart::Value("foo"),
            PatternPart::Joker,
            PatternPart::Joker,
            PatternPart::Value("bar"),
        ];
        let parts_as_usize: Vec<usize> = parts.iter().map(|part| part.into()).collect();
        let parts_control: Vec<usize> = vec![1, 0, 1, 1, 0,];
        assert_eq!(parts_as_usize, parts_control);
    }
}