// first idea: parition/ group by letter in advance. then we go around picking once from each group.
// itll be O(n) to group them. there's only twenty-six letters, so there's only twenty-six groups.

// keep track of 26 cooldowns, one for each letter. after completing a task, you decrement all other cooldowns,
// then you set its cooldown to `n`. to pick a new task, you look for one with zero cooldown,
// and pick a task out of the group for that letter.
// note that we have to worry about running out of tasks, potentially most cooldowns are ready but there aren't any
// tasks for that letter.
// we want to pick a group that has 1) zero cooldown, and 2) uncompleted tasks.

// map groups onto cooldowns and keep them in a vector
// keep vector length and gorups count in sync. when we deplete a group, kill that cooldown
// we just keep iterating over the array forerverand keep popping off the groups, in order.
// we do map over the array and decrement cooldowns by one every time. thats prolly not optimal. who cares.
// if there' sstill acooldown when we get atound to one group,
// we just add that to our result counter and then pop a task

const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct Solution;
impl Solution {
    /// You are given an array of CPU tasks, each represented by letters A to Z, and a cooling time, n.
    /// Each cycle or interval allows the completion of one task.
    /// Tasks can be completed in any order, but there's a constraint:
    /// identical tasks must be separated by at least n intervals due to cooling time.
    ///
    /// ​Return the minimum number of intervals required to complete all tasks.
    ///
    /// `0 <= n <= 100`
    ///
    /// `1 <= tasks.length <= 104`
    ///
    /// `tasks[i]` is an uppercase English letter.
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        // number of intervals thus far completed
        let mut n_intervals = 0;

        // number of tasks remaining for each character, A-Z
        let mut groups: [usize; 26] = [0; 26];
        for task in tasks {
            let idx = Self::alphabet_idx(&task);
            groups[idx] += 1;
        }

        // cooldowns are a tuple of (chracter: usize, cooldown: usize) where `character` is an index into the alphabet
        let mut cooldowns: Vec<(usize, usize)> = groups
            .iter()
            .enumerate()
            .filter(|(_, n)| **n != 0)
            .map(|(c, _)| (c, 0))
            .collect();

        println!("start!grps: {groups:?}");
        println!("start!cd: {cooldowns:?}");
        // until we finish all groups
        while !cooldowns.is_empty() {
            println!("llop!{cooldowns:?}");
            'cooldown: for (index, (c, cooldown)) in cooldowns.iter_mut().enumerate() {
                println!("task {c} gp:{}!", groups[*c]);
                // if we're out of tasks for this group
                if groups[*c] == 0 {
                    // don't try to complete tasks for this group anymore
                    cooldowns.remove(index);
                    break;
                }
                // wait for the cooldown
                if *cooldown != 0 {
                    n_intervals += *cooldown;
                }
                // perform task:
                // reset the cooldown
                *cooldown = n as usize;
                // advance time
                n_intervals += 1;
                for n_tasks in groups.iter_mut() {
                    // if we're out of tasks for this group
                    if *n_tasks <= 1 {
                        // don't try to complete tasks for this group anymore
                        cooldowns.remove(index);
                        break 'cooldown;
                    }
                    *n_tasks = *n_tasks - 1;
                }
            }
        }
        n_intervals as i32
    }

    fn alphabet_idx(c: &char) -> usize {
        // char must be A-Z. we don't check bc leetcode said it would be
        ALPHABET.iter().position(|a| a == c).unwrap()
    }
}

#[cfg(test)]
mod test {


    #[test]
    fn t1() {
        let tasks = vec!['A','A','A','B','B','B'];
        let n = 2;
        assert_eq!(super::Solution::least_interval(tasks, n), 8);
    }

    #[test]
    fn t2() {
        let tasks = vec!['A','C','A','B','D','B'];
        let n = 1;
        assert_eq!(super::Solution::least_interval(tasks, n), 6);
    }

    #[test]
    fn t3() {
        let tasks = vec!['A','A','A', 'B','B','B'];
        let n = 3;
        assert_eq!(super::Solution::least_interval(tasks, n), 10);
    }
}
