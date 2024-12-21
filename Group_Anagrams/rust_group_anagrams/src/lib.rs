use std::collections::HashMap;

pub fn group_anagrams<'a>(strs: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut sets: HashMap<[u8; 26], Vec<&str>> = HashMap::new();

    for str in strs {
        let str_set = str.chars().fold([0_u8; 26], |mut set, x| {
            if x.is_alphabetic() {
                let i = (x.to_ascii_lowercase() as usize) - 97;
                set[i] += 1;
            }
            set
        });

        if sets.contains_key(&str_set) {
            if let Some(value) = sets.get_mut(&str_set) {
                value.push(str);
            }
        } else {
            sets.insert(str_set, vec![str]);
        }
    }

    let mut result = sets.into_values().collect::<Vec<Vec<&'a str>>>();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let strs = ["act","pots","tops","cat","stop","hat"];
        let mut result = vec![vec!["act", "cat"],vec!["pots", "tops", "stop"],vec!["hat"]];
        result.sort();
        assert_eq!(result, group_anagrams(&strs));
    }
}
