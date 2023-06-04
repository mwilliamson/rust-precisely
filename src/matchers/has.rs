use crate::{Matcher, MatchResult, text_tree::TextTree};

pub struct HasMatcher<T, U, M> where M: Matcher<U> {
    name: String,
    extract: fn(T) -> U,
    matcher: M,
}

impl <T, U, M> Matcher<T> for HasMatcher<T, U, M> where M: Matcher<U> {
    fn match_value(&self, actual: T) -> MatchResult {
        let value = (self.extract)(actual);
        let result = self.matcher.match_value(value);

        match result {
            MatchResult::Matched => {
                MatchResult::matched()
            },
            MatchResult::Unmatched { explanation } => {
                MatchResult::unmatched(TextTree::nested(
                    TextTree::concat(vec![
                        TextTree::text(&self.name),
                        TextTree::text(" mismatched"),
                    ]),
                    explanation,
                ))
            },
        }
    }

    fn describe(&self) -> TextTree {
        TextTree::nested(
            TextTree::text(&self.name),
            self.matcher.describe()
        )
    }
}

pub fn has<T, U, M>(name: &str, extract: fn(T) -> U, matcher: M) -> HasMatcher<T, U, M> where M: Matcher<U> {
    HasMatcher {
        name: name.to_string(),
        extract,
        matcher: matcher,
    }
}

#[cfg(test)]
mod test {
    use crate::{matchers::{equal_to::equal_to, assertions::assert_unmatched}, Matcher, MatchResult};

    use super::has;

    struct User {
        username: String,
    }

    #[test]
    fn matches_when_feature_has_correct_value() {
        let matcher = has("name", |user: &User| &user.username, equal_to("bob"));

        let result = matcher.match_value(&User { username: "bob".to_string()});

        assert_eq!(result, MatchResult::matched());
    }

    #[test]
    fn explanation_of_mismatch_contains_mismatch_of_feature() {
        let matcher = has("name", |user: &User| &user.username, equal_to("bob"));

        let result = matcher.match_value(&User { username: "bobbity".to_string()});

        assert_unmatched(result, "name mismatched:\n  was \"bobbity\"");
    }

    #[test]
    fn description_contains_description_of_feature() {
        let matcher = has("name", |user: &User| &user.username, equal_to("bob"));

        let result = matcher.describe();

        assert_eq! (result.to_string(), "name:\n  \"bob\"");
    }
}
