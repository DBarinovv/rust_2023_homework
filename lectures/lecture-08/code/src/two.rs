use std::str::CharIndices;

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    let mut cur: Vec<_> = strs.iter().map(|str| str.char_indices()).collect();
    let res = String::new();
    let mut additional_iter = strs[0].char_indices();
    let mut cur_char = additional_iter.next();
    while let Some(ch) = cur_char {
        cur_char = additional_iter.next();
        if cur.iter().all(|iter: &mut CharIndices| iter.next() == cur_char) {
            res.push(ch);
        }
    }

    res
}
