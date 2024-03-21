    // /// A palindrome pair is a pair of integers (i, j)
    // /// words\[i] + words\[j] (the concatenation of the two strings) is a palindrome.
    // /// Return an array of all the palindrome pairs of words.
    // pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    //     // println!("weepa!");
    //     let mut ppairs = vec![];
    //     // construct a hashmap<char,string> of first characters to words ending with that key
    //     let ending_with = words.iter().enumerate().fold(
    //         HashMap::<char, Vec<(usize, &String)>>::new(),
    //         |mut hm, word_w_idx| {
    //             if word_w_idx.1.len() == 0 {
    //                 return hm;
    //             }
    //             hm.entry(*word_w_idx.1.as_bytes().last().unwrap() as char)
    //                 .or_insert_with(Vec::new)
    //                 .push(word_w_idx);
    //             hm
    //         },
    //     );
    //     // println!("ending_with: {{ending_with:?}}");
    //     for (word_idx, word) in words.iter().enumerate() {
    //         if word.len() == 0 {
    //             // todo
    //             ppairs.extend(
    //                 (0..words.len())
    //                     .filter_map(|o|{
    //                         if o == word_idx || !Self::is_palindrome(&words[o]) {
    //                             return None;
    //                         }
    //                         Some(vec![o as i32, word_idx as i32])
    //                     })
    //                     .chain((0..words.len()).filter_map(|o| {
    //                         if o == word_idx || !Self::is_palindrome(&words[o]) {
    //                             return None;
    //                         }
    //                         Some(vec![word_idx as i32, o as i32])
    //                     }))
    //             );
    //             // println!("extending for <{word}>, [{:?}, {word_idx}]", 0..words.len());
    //             continue;
    //         }
    //         // take the first character
    //         let first_letter = *word.as_bytes().first().unwrap() as char;
    //         // lookup in the hashmap, words ending with the first character
    //         let targets = ending_with
    //             .get(&first_letter)
    //             .map_or([].iter(), |t| t.iter());
    //         // println!("{word_idx}/ {}", words.len());
    //         // filter out those words who aren't palindromic with that letter
    //         let pdromes = targets.filter(|(target_word_idx, target_word)| {
    //             // no palindromes with self
    //             if *target_word_idx == word_idx {
    //                 return false;
    //             }

    //             match word.len().cmp(&target_word.len()) {
    //                 // if we run out of chars
    //                 Less => {
    //                     // we need the head of target to be palindromic.
    //                     let rest = &target_word[..target_word.len() - word.len()];
    //                     if !Self::is_palindrome(rest) {
    //                         // println!("rejected! we sml {word} :: {target_word} -- {rest}");
    //                         return false;
    //                     }
    //                 }
    //                 // if they roun out of chars
    //                 Greater => {
    //                     // we need the rest of us to be palindromic.
    //                     let rest = &word[target_word.len()..];
    //                     if !Self::is_palindrome(rest) {
    //                         // println!("rejected! we big {target_word} :: {word} -- {rest}");
    //                         return false;
    //                     }
    //                 }
    //                 _ => (),
    //             };

    //             // for each successive previous character (iterating backwards throught the target_word)
    //             for (idx, char) in target_word.bytes().rev().enumerate().skip(1) {
    //                 // typecast byte to char
    //                 let char = char as char;
    //                 // println!("{word}: {char}@{idx} for {target_word}");
    //                 // if the word finishes
    //                 if word.len() < idx + 1 {
    //                     // println!("{word}: {target_word} lngr than us ");
    //                     return true;
    //                 }
    //                 let Some(word_char) = word.as_bytes().iter().nth(idx) else {
    //                     // println!("rejected! {word}: @{idx} null ");
    //                     return false;
    //                 };
    //                 // println!("{word}: -- {target_word}: {char}<>{} @{idx} test...", *word_char as char);
    //                 if *word_char as char != char {
    //                     // println!("Bd! {word} -- {target_word}: {char} @{idx} ne ");
    //                     return false;
    //                 }
    //             }
    //             // println!("{word}: {target_word} fallthru");
    //             true
    //         });
    //         // add those that remain
    //         // println!("pdroms for {word} are {:?}", pdromes.clone().collect::<Vec<_>>());
    //         for pdrome in pdromes {
    //             // println!("pdrom for {word} are {pdrome:?}");
    //             ppairs.push(vec![word_idx as i32, pdrome.0 as i32])
    //         }
    //         // println!("PPAIRS {word} {ppairs:?}");
    //     }
    //     ppairs
    // }