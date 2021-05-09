///用于找到字符串中需要使用变量替换的字符串
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
