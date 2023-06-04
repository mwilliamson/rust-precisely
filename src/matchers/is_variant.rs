use crate::{Matcher, MatchResult, text_tree::TextTree};

#[macro_export]
macro_rules! is_variant {
    ($variant:pat) => {
        crate::matchers::is_variant::VariantMatcher {
            extract: |actual| {
                match actual {
                    $variant => Some(&()),
                    _ => None,
                }
            },
            matchers: vec![],
        }
    };

    ($variant:pat => $feature:expr, $($matchers:expr),* $(,)?) => {
        crate::matchers::is_variant::VariantMatcher {
            extract: |actual| {
                match actual {
                    $variant => Some($feature),
                    _ => None,
                }
            },
            matchers: vec![$(Box::new($matchers),)*],
        }
    };
}

struct VariantMatcher<T, U> where U: Copy {
    extract: fn(T) -> Option<U>,
    matchers: Vec<Box<dyn Matcher<U>>>,
}

impl <T, U> Matcher<T> for VariantMatcher<T, U> where U: Copy {
    fn match_value(&self, actual: T) -> MatchResult {
        let value = (self.extract)(actual);

        match value {
            None => {
                MatchResult::unmatched(TextTree::text(""))
            }
            Some(value) => {
                for matcher in &self.matchers {
                    let result = matcher.match_value(value);

                    if let MatchResult::Unmatched { .. } = result {
                        return result
                    }
                }
                MatchResult::matched()
            }
        }
    }

    fn describe(&self) -> TextTree {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{MatchResult, Matcher, matchers::{assertions::assert_unmatched, has::has, equal_to::equal_to}};

    #[test]
    fn can_match_singleton_variant() {
        #[allow(unused)]
        enum X {
            X1,
            X2,
        }

        let matcher = is_variant!(X::X1);

        let result = matcher.match_value(X::X1);

        assert_eq!(result, MatchResult::matched());
    }

    #[test]
    fn can_match_tuple_variant_with_no_fields() {
        #[allow(unused)]
        enum X {
            X1(),
            X2,
        }

        let matcher = is_variant!(X::X1());

        let result = matcher.match_value(X::X1());

        assert_eq!(result, MatchResult::matched());
    }

    #[test]
    fn can_match_tuple_variant_with_single_field() {
        #[allow(unused)]
        enum X {
            X1(i32),
            X2,
        }

        let matcher = is_variant!(X::X1(_));

        let result = matcher.match_value(X::X1(42));

        assert_eq!(result, MatchResult::matched());
    }

    #[test]
    fn can_match_tuple_variant_with_many_fields() {
        #[allow(unused)]
        enum X {
            X1(i32, i32, i32),
            X2,
        }

        let matcher = is_variant!(X::X1(_, _, _));

        let result = matcher.match_value(X::X1(1, 2, 3));

        assert_eq!(result, MatchResult::matched());
    }

    #[test]
    fn when_variant_and_submatchers_match_then_overall_matcher_matches() {
        #[allow(unused)]
        enum X {
            X1(i32, i32, i32),
            X2,
        }

        let matcher = is_variant!(
            X::X1(a, b, c) => (a, b, c),
            has(".0", |x: (i32, i32, i32)| x.0, equal_to(1)),
            has(".1", |x: (i32, i32, i32)| x.1, equal_to(2)),
            has(".2", |x: (i32, i32, i32)| x.2, equal_to(3)),
        );

        let result = matcher.match_value(X::X1(1, 2, 3));

        assert_eq!(result, MatchResult::matched());
    }

    #[test]
    fn when_any_submatcher_does_not_match_then_overall_matcher_mismatches() {
        #[allow(unused)]
        enum X {
            X1(i32, i32, i32),
            X2,
        }
        let matcher = is_variant!(
            X::X1(a, b, c) => (a, b, c),
            has(".0", |x: (i32, i32, i32)| x.0, equal_to(1)),
            has(".1", |x: (i32, i32, i32)| x.1, equal_to(4)),
            has(".2", |x: (i32, i32, i32)| x.2, equal_to(3)),
        );

        let result = matcher.match_value(X::X1(1, 2, 3));

        assert_unmatched(result, ".1 mismatched:\n  was 2");
    }
}
