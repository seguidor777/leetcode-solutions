use leetcode_solutions::word_ladder;

#[test]
fn word_ladder_test() {
    let res = word_ladder::Solution::ladder_length(
        "hit".to_string(),
        "cog".to_string(),
        vec!["hot", "dot", "dog", "lot", "log", "cog"]
            .iter()
            .map(|&x| x.into())
            .collect(),
    );

    assert_eq!(5, res);
}
