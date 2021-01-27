use crate::UriPattern;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub(crate) struct UriPatternScore {
    score: usize,
}

impl From<&UriPattern<'_>> for UriPatternScore {
    fn from(pattern: &UriPattern) -> Self {
        let score: Vec<usize> = pattern.parts.iter().map(|part| part.into()).collect();
        let len = score.len() as u32 - 1u32;
        let score: usize = score.iter().enumerate().fold(0, |sum, (k, v)| sum + v * 10usize.pow(len - k as u32));
        UriPatternScore {score}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_for_uri_pattern() {
        let pattern: &UriPattern = &"/a/{b}/{c}/d".into();
        let score: UriPatternScore = pattern.into();
        assert_eq!(score.score, 110);

        let pattern: &UriPattern = &"/{a}/b/c/d".into();
        let score: UriPatternScore = pattern.into();
        assert_eq!(score, UriPatternScore{ score: 1000});
    }
}