// use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;
#[derive(Debug)]
struct Trie<'a> {
    // if nodes up to this point make up an input word, (idx, &word)
    word: Option<(usize, &'a String)>,
    // optimization: make this an option, so that our leafs don't have uselesss empty hashmaps
    m: HashMap<char, Trie<'a>>,
}
impl Trie<'_> {
    fn new<'a>() -> Trie<'a> {
        Trie {
            word: None,
            m: HashMap::new(),
        }
    }
}
pub struct Solution;
impl Solution {
    /// A palindrome pair is a pair of integers (i, j)
    /// words\[i] + words\[j] (the concatenation of the two strings) is a palindrome.
    /// Return an array of all the palindrome pairs of words.
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut ppairs = vec![];

        let mut tree = Trie::new();
        for (idx, word) in words.iter().enumerate() {
            let mut node = &mut tree;
            for char in word.as_bytes().iter().rev() {
                let char = *char as char;
                node = node.m.entry(char).or_insert_with(Trie::new)
            }
            node.word = Some((idx, word));
        }

        // walk the tree
        for (word_idx, word) in words.iter().enumerate() {
            // palindromes with ""
            if word.len() == 0 {
                ppairs.extend(
                    (0..words.len())
                        .filter_map(|o| {
                            if o == word_idx || !Self::is_palindrome(&words[o]) {
                                return None;
                            }
                            Some(vec![o as i32, word_idx as i32])
                        })
                        .chain((0..words.len()).filter_map(|o| {
                            if o == word_idx || !Self::is_palindrome(&words[o]) {
                                return None;
                            }
                            Some(vec![word_idx as i32, o as i32])
                        })),
                );
                // println!("extending for <{word}>, [{:?}, {word_idx}]", 0..words.len());
                continue;
            }

            // whether we ran out of nodes or we completed the loop normally
            let mut depleted = false;
            // this reference progressses down the tree
            let mut node = &mut tree;
            for char in word.as_bytes() {
                let char = *char as char;
                if node.m.get_mut(&char).is_some() {
                    node = node.m.get_mut(&char).unwrap();
                } else {
                    depleted = true;
                    break;
                }
                if let Some((target_idx, target_word)) = node.word {
                    let rest = &word[target_word.len()..];
                    // case we are longer, check if rest is palindrome
                    if word_idx == target_idx || !Self::is_palindrome(rest) {
                        continue;
                    }
                    ppairs.push(vec![word_idx as i32, target_idx as i32]);
                }
            }
            if depleted {
                continue
            }
            // check the rest of the tree
            // println!("word:{word} node:{node:?}");
            let mut node_q = vec![&*node];
            while let Some(node) = node_q.pop() {
                if let Some((target_idx, target_word)) = node.word {
                    let rest = &target_word[..target_word.len() - word.len()];
                    // case they are longer, check if rest is palindrome
                    if word_idx != target_idx && Self::is_palindrome(rest) {
                        // println!("pushing [{word_idx}, {target_idx}]");
                        ppairs.push(vec![word_idx as i32, target_idx as i32]);
                    }
                }
                node_q.extend(node.m.values());
            }
        }
        ppairs
    }

    fn is_palindrome(s: &str) -> bool {
        let mid = s.len() / 2;
        let even_len = s.len() % 2 == 0;
        let front_iter = s.as_bytes().iter().take(mid);
        let back_iter = s
            .as_bytes()
            .iter()
            .rev()
            .take(if even_len { mid } else { mid + 1 });
        for (c1, c2) in front_iter.zip(back_iter) {
            let (c1, c2) = (*c1 as char, *c2 as char);
            if c1 != c2 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::Solution;
    #[test]
    fn t1() {
        let words: Vec<_> = ["abcd", "dcba", "lls", "s", "sssll"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        let ppairs: HashSet<_> = [[0, 1], [1, 0], [3, 2], [2, 4]]
            .iter()
            .map(|pair| pair.to_vec())
            .collect();
        assert_eq!(
            HashSet::from_iter(Solution::palindrome_pairs(words)),
            ppairs
        )
    }

    #[test]
    fn t2() {
        let words: Vec<_> = ["bat", "tab", "cat"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        let ppairs: HashSet<_> = [[0, 1], [1, 0]].iter().map(|pair| pair.to_vec()).collect();
        assert_eq!(
            HashSet::from_iter(Solution::palindrome_pairs(words)),
            ppairs
        )
    }

    #[test]
    fn t3() {
        let words: Vec<_> = ["a", ""].iter().map(|s| (*s).to_owned()).collect();
        let ppairs: HashSet<_> = [[0, 1], [1, 0]].iter().map(|pair| pair.to_vec()).collect();
        assert_eq!(
            HashSet::from_iter(Solution::palindrome_pairs(words)),
            ppairs
        )
    }

    #[test]
    fn t4() {
        //  extra: `[[0, 2], [1, 3], [3, 1]]`,
        let words: Vec<_> = ["a", "abc", "aba", ""]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        let ppairs: HashSet<_> = [[0, 3], [3, 0], [2, 3], [3, 2]]
            .iter()
            .map(|pair| pair.to_vec())
            .collect();
        assert_eq!(
            HashSet::from_iter(Solution::palindrome_pairs(words)),
            ppairs
        )
    }

    #[test]
    fn t5() {
        //  extra: `[[0, 2], [1, 3], [3, 1]]`,
            .iter().map(|s| (*s).to_owned()).collect();
        let ppairs: HashSet<_> = [
            [26, 676],
            [52, 1352],
            [78, 2028],
            [104, 2704],
            [130, 3380],
            [156, 4056],
            [182, 4732],
            [676, 26],
            [728, 1378],
            [754, 2054],
            [780, 2730],
            [806, 3406],
            [832, 4082],
            [858, 4758],
            [1352, 52],
            [1378, 728],
            [1430, 2080],
            [1456, 2756],
            [1482, 3432],
            [1508, 4108],
            [1534, 4784],
            [2028, 78],
            [2054, 754],
            [2080, 1430],
            [2132, 2782],
            [2158, 3458],
            [2184, 4134],
            [2210, 4810],
            [2704, 104],
            [2730, 780],
            [2756, 1456],
            [2782, 2132],
            [2834, 3484],
            [2860, 4160],
            [2886, 4836],
            [3380, 130],
            [3406, 806],
            [3432, 1482],
            [3458, 2158],
            [3484, 2834],
            [3536, 4186],
            [3562, 4862],
            [4056, 156],
            [4082, 832],
            [4108, 1508],
            [4134, 2184],
            [4160, 2860],
            [4186, 3536],
            [4238, 4888],
            [4732, 182],
            [4758, 858],
            [4784, 1534],
            [4810, 2210],
            [4836, 2886],
            [4862, 3562],
            [4888, 4238],
        ]
        .iter()
        .map(|pair| pair.to_vec())
        .collect();
        assert_eq!(
            HashSet::from_iter(Solution::palindrome_pairs(words)),
            ppairs
        )
    }
}

#[cfg(test)]
mod palindrome {
    use super::Solution;
    #[test]
    fn t1() {
        assert!(Solution::is_palindrome("ala"))
    }
    #[test]
    fn t2() {
        assert!(Solution::is_palindrome("alla"))
    }
    #[test]
    fn t3() {
        assert!(!Solution::is_palindrome("alba"))
    }
    #[test]
    fn t4() {
        assert!(!Solution::is_palindrome("ak"))
    }
    #[test]
    fn t5() {
        assert!(Solution::is_palindrome("a"))
    }
    #[test]
    fn t6() {
        assert!(Solution::is_palindrome(""))
    }
}