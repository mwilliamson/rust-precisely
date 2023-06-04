use crate::{Matcher, text_tree::TextTree, MatchResult};

pub fn equal_to<T>(value: T) -> EqualToMatcher<T> where T: PartialEq {
    EqualToMatcher { value }
}

pub struct EqualToMatcher<T> {
    value: T,
}

impl <T> Matcher<T> for EqualToMatcher<T> where T: std::fmt::Debug + PartialEq {
    fn match_value(&self, actual: T) -> MatchResult {
        if self.value == actual {
            MatchResult::matched()
        } else {
            MatchResult::unmatched(TextTree::concat(vec![
                TextTree::text("was "),
                TextTree::debug(actual)
            ]))
        }
    }

    fn describe(&self) -> TextTree {
        TextTree::debug(&self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::{Matcher, MatchResult, matchers::assertions::assert_unmatched};

    use super::equal_to;

    #[test]
    fn matches_when_values_are_equal() {
        let matcher = equal_to(1);

        let result = matcher.match_value(1);

        assert_eq!(result, MatchResult::Matched);
    }

    #[test]
    fn explanation_of_mismatch_contains_debug_string_of_actual() {
        let matcher = equal_to(1);

        let result = matcher.match_value(2);

        assert_unmatched(result, "was 2");
    }

    #[test]
    fn description_is_debug_string_of_value() {
        let matcher = equal_to(1);

        let result = matcher.describe();

        assert_eq!(result.to_string(), "1");
    }
}
