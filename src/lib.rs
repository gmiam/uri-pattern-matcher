//! This crate can be used to parse URIs like the ones we can found in OpenApi spec for paths (/foo/{bar}).
//! Once the pattern is parsed, you can check if any string matches against it. You can also compare two patterns to find the more specific.
//!
//! For now it doesn't handle any other pattern than {pattern}. Feel free to open an issue if you have a need for a specific usecase.
//! Can probably be used for paths on filesystems as well if One can find a usecase for this.
//!
//! # Example
//!
//! Here is examples for the obvious usages of this crate:
//!
//! ```rust
//! # use uri_pattern_matcher::UriPattern;
//! let pattern: UriPattern = "/api/{resource}/{id}/details".into();
//! assert!(pattern.is_match("/api/resource/id1/details"));
//! assert!(pattern.is_match("/api/customer/John/details"));
//! ```
//!
//! ```rust
//! # use uri_pattern_matcher::UriPattern;
//! let pattern: UriPattern = "/api/{foo}/{bar}/zzz".into();
//! let pattern2: UriPattern = "/api/{foo}/bar/{zzz}".into();
//! assert_ne!(pattern, pattern2);
//! assert!(pattern > pattern2);
//! ```
//!
//! We are also able combine all of this using Iterators
//! Here we'll retrieve the most specific pattern matching our candidate string
//! ```rust
//! // we use this because fold_first is behind this flag and on nightly only
//! #![feature(iterator_fold_self)]
//! # use uri_pattern_matcher::UriPattern;
//! let patterns: Vec<UriPattern> = vec![
//!     "/api/{foo}/{bar}/zzz".into(),
//!     "/api/{foo}/bar/{zzz}".into(),
//!     "/{api}/{foo}/foo/{zzz}".into()
//!     ];
//! let candidate = "/api/resource/bar/zzz";
//! let best_match = patterns.iter()
//!            .filter(|p| p.is_match(candidate))
//!            .fold_first(|a, b| {
//!            if a >= b { a } else { b }
//!            });
//! assert_eq!(best_match.unwrap(), &UriPattern::from("/api/{foo}/{bar}/zzz"))
//! ```
mod pattern_part;
mod uri_pattern_score;

use core::cmp::Ordering;
use crate::pattern_part::PatternPart;
use crate::uri_pattern_score::UriPatternScore;

/// Struct used to parse strings as patterns - Check if an incoming string matches a pattern - Pattern Comparison
#[derive(Debug, Clone)]
pub struct UriPattern<'a> {
    value: &'a str,
    pub(crate) parts: Vec<PatternPart<'a>>,
}

impl<'a> From<&'a str> for UriPattern<'a> {
    fn from(pattern: &'a str) -> Self {
        let parts = pattern.split('/').map(|part| part.into()).collect();
        Self { value : pattern, parts }
    }
}

impl UriPattern<'_> {
    /// Method used to check if a candidate string matches against the pattern
    /// # Example
    ///
    /// ```rust
    /// use uri_pattern_matcher::UriPattern;
    ///
    /// let pattern: UriPattern = "/api/{resource}/{id}/details".into();
    /// assert!(pattern.is_match("/api/resource/id1/details"));
    /// assert!(pattern.is_match("/api/customer/John/details"));
    /// ```
    pub fn is_match(&self, candidate: &str) -> bool {
        !candidate.split('/').enumerate().map(|(key, value)| {
            match self.parts[key] {
                PatternPart::Joker => true,
                PatternPart::Value(s) => if s == value {true} else {false},
            }
        })
            .collect::<Vec<bool>>()
            .contains(&false)
    }
}

impl PartialEq for UriPattern<'_> {
    fn eq(&self, other: &Self) -> bool {
        let score: UriPatternScore = self.into();
        let other_score: UriPatternScore = other.into();
        score == other_score
    }
}

impl PartialOrd for UriPattern<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let score: UriPatternScore = self.into();
        let other_score: UriPatternScore = other.into();
        score.partial_cmp(&other_score)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let pattern: UriPattern = "/a/{b}/{c}/d".into();
        assert!(pattern.is_match("/a/resource/test/d"));
    }

    #[test]
    fn non_equality_works() {
        let pattern: UriPattern = "/a/{b}/{c}/d".into();
        let pattern2: UriPattern = "/a/{b}/c/{d}".into();
        assert_ne!(pattern, pattern2);
    }

    #[test]
    fn equality_works() {
        let pattern: UriPattern = "/a/{b}/{c}/d".into();
        let pattern2: UriPattern = "/api/{resource}/{id}/details".into();
        assert_eq!(pattern, pattern2);
    }

    #[test]
    fn inequality_works() {
        let pattern: UriPattern = "/a/{b}/{c}/d".into();
        let pattern2: UriPattern = "/a/{b}/c/{d}".into();
        assert!(pattern > pattern2);
    }
}
