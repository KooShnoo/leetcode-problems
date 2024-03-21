use std::collections::HashMap;
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_window(s: String, t: String) -> String {
    
        if t.len() == 1 {
            return if s.contains(&t) {
                t
            } else {
                "".to_owned()
            }
        }
    
        // windows are calculated with .. range (inclusive..exclusive)
        let mut min_win = (0, s.len());
        let (mut left, mut right) = (0, 0);
        // let mut num_chars_not_in_window = t.len();
        let (mut chars_in_window, target) = t.as_bytes().iter().fold((HashMap::<char, usize>::new(), HashMap::<char, usize>::new()), |tup, char| {
            // let mut num_chars_not_in_window = t.len();
            let (mut hm, mut target) = tup;
            let char = *char as char;
            if target.contains_key(&char) {
                target.entry(char).and_modify(|num_char| *num_char += 1);
            } else {
                target.insert(char as char, 1);
            }
            hm.insert(char as char, 0);
            (hm, target)
        });
        // this is sloppy
        let mut found_sol = false;
        dbg!(s.len(), &target);
        println!("l={left}, r={right} chars_in_window: {:?}", &chars_in_window);
    
        let mut left_ptr_crossed_target_char = false;
    
        'outer: while left <= s.len() {
            if right < s.len() {
                let next_char = s.as_bytes()[right] as char;
                if chars_in_window.contains_key(&next_char) {
                    chars_in_window.entry(next_char).and_modify(|num_chars|{*num_chars += 1;});
                    // num_chars_not_in_window -= 1;
                }
                right += 1;
            }
    // move left ptr if window is valid, or if right ptr is at the end
        // stop moving left ptr if we hit a target
            
            println!("l={left}, r={right} minwin{min_win:?} chars_in_window: {:?} win: {}", &chars_in_window, (s[left..right]).to_owned());
            if Self::all_greater_than_target(&chars_in_window, &target) || right >= s.len() {
                if Self::all_greater_than_target(&chars_in_window, &target) {
                    found_sol = true;
                    if min_win.1 - min_win.0 > right - left {
                        min_win = (left, right);
                    }
                }
                println!("{}, {}", min_win.1 - min_win.0, right - left);
                loop {
                    if min_win.1 - min_win.0 > right - left && Self::all_greater_than_target(&chars_in_window, &target) {
                        min_win = (left, right);
                    }
                    if left >= s.len() - 1 {
                        println!("epo{} {left} {right}", s[left..right].to_owned());
                        println!("epo{min_win:?} {chars_in_window:?}");
                        break 'outer;
                    }
                    println!("left: {left}, win:{}, left_ptr_crossed_target_char{left_ptr_crossed_target_char} ", (s[left..right]).to_owned());
                    // stop if the next character is in the target.
                    let next_char = s.as_bytes()[left] as char;
                    if chars_in_window.contains_key(&next_char) {
                        if right < s.len() && !Self::all_greater_than_target(&chars_in_window, &target) {
                            left_ptr_crossed_target_char = false;
                            break;
                        }
                        chars_in_window.entry(next_char).and_modify(|num_chars|{*num_chars -= 1;});
                        left_ptr_crossed_target_char = true;
                    }
                    left += 1;
                }
            }
    
    
        }
        if found_sol {
            s[min_win.0..min_win.1].into()
        } else {
            "".to_owned()
        }
    }
    
    /// oof i can't name things. this checks we have reached or exceeded our target count of characters
    /// in our window.
    fn all_greater_than_target(inp: &HashMap::<char, usize>, target: &HashMap::<char, usize>) -> bool {
        inp.keys().all(|char| {
            inp[char] >= target[char]
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn t1() {
        assert_eq!(Solution::min_window("ADOBECODEBANC".into(), "ABC".into()), "BANC".to_owned());
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::min_window("a".into(), "a".into()), "a".to_owned());
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::min_window("a".into(), "aa".into()), "".to_owned());
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::min_window("ab".into(), "b".into()), "b".to_owned());
    }

    #[test]
    fn t5() {
        assert_eq!(Solution::min_window("abc".into(), "b".into()), "b".to_owned());
    }

    #[test]
    fn t6() {
        assert_eq!(Solution::min_window("abc".into(), "bc".into()), "bc".to_owned());
    }

    #[test]
    fn t7() {
        assert_eq!(Solution::min_window("adobecodebancbbcaa".into(), "abc".into()), "bca".to_owned());
    }
}