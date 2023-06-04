use crate::MatchResult;

pub(crate) fn assert_unmatched(result: MatchResult, expected_explanation: &str) {
    match result {
        MatchResult::Matched => {
            panic!("expected unmatched, got matched");
        },
        MatchResult::Unmatched { explanation } => {
            assert_eq!(explanation.to_string(), expected_explanation);
        },
    }
}
