use rcicd_helper::str;

#[test]
fn test_find_tag_variables() {
    let vars1 =
        str::find_tag_variables("{proj_domain_prefix}.{proj_top_domain} test str1", '{', '}');
    assert_eq!(vars1, vec!["proj_domain_prefix", "proj_top_domain"]);
    let vars2 = str::find_tag_variables("test #test_var1##test_var2#hello word", '#', '#');
    assert_eq!(vars2, vec!["test_var1", "test_var2"]);
}
