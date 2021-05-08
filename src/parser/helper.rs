pub fn find_tag_variables(src: &str, start_tag: char, end_tag: char) -> Vec<&str> {
    let mut find = false;
    let mut start_index = 0;
    src.chars()
        .enumerate()
        .filter_map(|(index, c)| {
            if c == start_tag && find == false {
                find = true;
                start_index = index + 1;
                return None;
            }

            if c.is_whitespace() {
                find = false;
            }

            if c == end_tag && find {
                dbg!((start_index, index));
                let res = Some(&src[start_index..index]);
                find = false;
                return res;
            }
            None
        })
        .collect::<Vec<&str>>()
}

#[test]
fn test_find_tag_variables() {
    let vars1 = find_tag_variables("{proj_domain_prefix}.{proj_top_domain} test str1", '{', '}');
    assert_eq!(vars1, vec!["proj_domain_prefix", "proj_top_domain"]);
    let vars2 = find_tag_variables("test #test_var1##test_var2#hello word", '#', '#');
    assert_eq!(vars2, vec!["test_var1", "test_var2"]);
}
