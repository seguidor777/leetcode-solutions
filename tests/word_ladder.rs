use leetcode_prelude::vec_string;
use leetcode_solutions::word_ladder;

#[test]
fn word_ladder_test_case_1() {
    let res = word_ladder::Solution::ladder_length(
        "hit".to_string(),
        "cog".to_string(),
        vec_string!["hot", "dot", "dog", "lot", "log", "cog"],
    );

    assert_eq!(5, res);
}

#[test]
fn word_ladder_test_case_2() {
    let res = word_ladder::Solution::ladder_length(
        "hit".to_string(),
        "cog".to_string(),
        vec_string!["hot", "dot", "dog", "lot", "log"],
    );

    assert_eq!(0, res);
}
