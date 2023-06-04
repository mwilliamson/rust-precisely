use text_tree::TextTree;

pub mod matchers;
pub mod text_tree;

/// The result of trying to match a value.
#[derive(Debug, PartialEq)]
pub enum MatchResult {
    Matched,
    Unmatched {
        explanation: TextTree,
    },
}

impl MatchResult {
    pub fn matched() -> MatchResult {
        MatchResult::Matched
    }

    pub fn unmatched(explanation: TextTree) -> MatchResult {
        MatchResult::Unmatched { explanation }
    }
}

pub trait Matcher<T> {
    fn match_value(&self, value: T) -> MatchResult;
    fn describe(&self) -> TextTree;
}

/// Assert that a value satisfies a matcher. On failure, panic with an
/// explanation of the mismatch.
pub fn assert_that<T, M>(value: T, matcher: M) where M : Matcher<T> {
    let result = matcher.match_value(value);

    if let MatchResult::Unmatched { explanation } = result {
        let error_description = TextTree::lines(vec![
            TextTree::text(""),
            TextTree::nested(TextTree::text("Expected"), matcher.describe()),
            TextTree::nested(TextTree::text("but"), explanation),
        ]);
        panic!("{}", error_description);
    }
}

#[cfg(test)]
mod test {
    use crate::{assert_that, matchers::equal_to::equal_to};

    #[test]
    fn assert_that_returns_normally_if_matcher_matches() {
        assert_that(1, equal_to(1));
    }

    #[test]
    #[should_panic(expected = "\nExpected:\n  2\nbut:\n  was 1")]
    fn assert_that_panics_if_match_fails() {
        assert_that(1, equal_to(2));
    }
}
