use crate::{Matcher, text_tree::TextTree, MatchResult};

pub struct AnythingMatcher {

}

impl AnythingMatcher {
    fn describe(&self) -> TextTree {
        TextTree::text("anything")
    }
}

impl <T> Matcher<T> for AnythingMatcher {
    fn match_value(&self, _: T) -> MatchResult {
        MatchResult::matched()
    }

    fn describe(&self) -> TextTree {
        self.describe()
    }
}

pub fn anything() -> AnythingMatcher {
    AnythingMatcher {}
}

#[cfg(test)]
mod test {
    use crate::{MatchResult, matchers::anything::anything, Matcher};

    #[test]
    fn matches_anything() {
        let matcher = anything();

        assert_eq!(matcher.match_value(4), MatchResult::matched());
        assert_eq!(matcher.match_value("hello"), MatchResult::matched());
    }

    #[test]
    fn description_is_anything() {
        let matcher = anything();

        let result = matcher.describe();

        assert_eq!(result.to_string(), "anything");
    }
}
