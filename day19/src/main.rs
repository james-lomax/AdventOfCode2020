use regex::{Regex, Captures};
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
    Or(Vec<usize>, Vec<usize>),
    Seq(Vec<usize>),
    Char(char)
}

fn parse_nums(line: &str) -> Vec<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]+")
            .expect("Regex compile failed");
    }
    RE.find_iter(line)
        .map(|m| m.as_str().parse::<usize>()
            .expect("[0-9]+ matching string parse failed"))
        .collect()
}

fn expect_cap<'a>(cap: &Captures<'a>, i: usize) -> &'a str {
    cap.get(i).expect("Defined capture group not present in match").as_str()
}

fn parse_rule(line: &str) -> (usize, Rule) {
    lazy_static! {
        static ref OR_R: Regex = 
            Regex::new(r"^([0-9]+): (([0-9 ]+)*) \| (([0-9 ]+)*)$").expect("Regex compile failed");
        static ref SEQ_R: Regex = 
            Regex::new(r"^([0-9]+):(( [0-9]+)*)$").expect("Regex compile failed");
        static ref CHAR_R: Regex = 
            Regex::new(r###"^([0-9]+): "([a-z])"$"###).expect("Regex compile failed");
    }

    if let Some(cap) = SEQ_R.captures(line) {
        (
            expect_cap(&cap, 1).parse::<usize>().unwrap(),
            Rule::Seq(parse_nums(expect_cap(&cap, 2)))
        )
    } else if let Some(cap) = OR_R.captures(line) {
        (
            expect_cap(&cap, 1).parse::<usize>().unwrap(),
            Rule::Or(parse_nums(expect_cap(&cap, 2)), parse_nums(expect_cap(&cap, 4)))
        )
    } else if let Some(cap) = CHAR_R.captures(line) {
        (
            expect_cap(&cap, 1).parse::<usize>().unwrap(),
            Rule::Char(expect_cap(&cap, 2).chars().nth(0)
                .expect("Regex has failed us."))
        )
    } else {
        panic!("Rule line does not match any known format!");
    }
}

fn parse_rules(contents: &str) -> HashMap<usize, Rule> {
    contents.split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(parse_rule)
        .collect()
}

#[derive(Clone)]
struct PatternMachine {
    rules: HashMap<usize, Rule>,
    history: Vec<(usize, usize, usize)>, // History stack of rule, index, branch
    rule: usize,  // Current rule
    index: usize, // Index within the sequence
    branch: usize,
    branch_chosen: bool,
    end: bool
}

impl PatternMachine {
    fn new(rules: HashMap<usize, Rule>) -> Self {
        Self {
            rules: rules,
            history: Vec::new(),
            index: 0,
            rule: 0,
            branch: 0,
            branch_chosen: false,
            end: false
        }
    }

    fn is_end(&self) -> bool {
        self.end
    }

    fn is_branch(&self) -> bool {
        let rule = self.rules.get(&self.rule).expect(format!("Bad rule index {}!", self.rule).as_str());
        match rule {
            Rule::Or(_, _) => true,
            _ => false
        }
    }

    // Take a branch, choosing branch index (0 or 1)
    fn take_branch(&mut self, branch: usize) {
        if branch == 0 || branch == 1 {
            self.branch = branch;
            self.branch_chosen = true;
        } else {
            panic!("Branch must be 0 or 1");
        }
    }

    fn push_r(&mut self) {
        self.branch_chosen = false;
        self.history.push((
            self.rule,
            self.index,
            self.branch
        )); 
    }

    fn pop_r(&mut self) -> bool {
        if let Some((r, i, b)) = self.history.pop() {
            self.rule = r;
            self.index = i;
            self.branch = b;
            self.branch_chosen = true;
            return true;
        } else {
            return false;
        }
    }

    // Step the sequence, return true to continue, false means done.
    fn step_seq(&mut self, seq: &Vec<usize>) -> bool {
        if self.index < seq.len() {
            // Investigate next rule in sequence
            let old_idx = self.index;
            self.index += 1;
            self.push_r();
            self.rule = seq[old_idx];
            self.index = 0;
        } else {
            // Done here
            if !self.pop_r() {
                // Nothing left to explore.. Finish.
                return false;
            }
        }

        return true;
    }

    fn next(&mut self) -> Option<char> {
        loop {
            let rule = self.rules.get(&self.rule).expect(format!("Bad rule index {}!", self.rule).as_str()).clone();
            match rule {    
                Rule::Or(b0, b1) => {
                    if self.branch_chosen {
                        let b = if self.branch == 0 {
                            b0
                        } else {
                            b1
                        };
                        if !self.step_seq(&b) {
                            self.end = true;
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                Rule::Seq(s) => {
                    if !self.step_seq(&s) {
                        self.end = true;
                        return None;
                    }
                }
                Rule::Char(c) => {
                    // Done here, pop one and return
                    self.pop_r();
                    return Some(c);
                }
            }
        }
    }
}

fn r_match(mut machine: PatternMachine, line: &str) -> bool {
    if let Some(c) = machine.next() {
        if line.len() == 0 {
            // Expected to read but didn't
            false
        } else if c == line.chars().nth(0).unwrap() {
            // Read fine, continue
            r_match(machine, &line[1..])
        } else {
            // Read wrong character, end
            false
        }
    } else {
        if machine.is_end() {
            // End, expect line to be empty now
            line.len() == 0
        } else if machine.is_branch() {
            // Branch, split the machine in two
            let mut m0 = machine.clone();
            m0.take_branch(0);
            let mut m1 = machine;
            m1.take_branch(1);
            r_match(m0, line) || r_match(m1, line)
        } else {
            panic!("Machine ended without branch/end reason!");
        }
    }
}

fn rule_match(rules: &HashMap<usize, Rule>, line: &str) -> bool {
    r_match(PatternMachine::new(rules.clone()), line)
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");
    let sections: Vec<&str> = contents.split("\n\n").collect();
    assert_eq!(sections.len(), 2);

    let mut rules = parse_rules(sections[0]);
    let lines: Vec<&str> = sections[1]
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .collect();
    
    let p1_count = lines.iter().filter(|l| rule_match(&rules, l)).count();
    println!("Part 1 count = {}", p1_count);

    // Modified rules for part 2
    rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));

    let p2_count = lines.iter().filter(|l| rule_match(&rules, l)).count();
    println!("Part 2 count = {}", p2_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let sample = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"";
        let rules = parse_rules(sample);
        assert_eq!(rules.get(&0), Some(&Rule::Seq(vec![4, 1, 5])));
        assert_eq!(rules.get(&1), Some(&Rule::Or(vec![2, 3], vec![3, 2])));
        assert_eq!(rules.get(&4), Some(&Rule::Char('a')));

        let mut machine = PatternMachine::new(rules.clone());
        assert_eq!(machine.next(), Some('a'));
        assert_eq!(machine.next(), None);
        assert!(machine.is_branch());
        machine.take_branch(1);
        assert_eq!(machine.next(), None);
        machine.take_branch(0);
        assert_eq!(machine.next(), Some('a'));
        assert_eq!(machine.next(), Some('b'));

        assert!(rule_match(&rules, "ababbb"));
        assert!(!rule_match(&rules, "bababa"));
        assert!(rule_match(&rules, "abbbab"));
        assert!(!rule_match(&rules, "aaabbb"));
        assert!(!rule_match(&rules, "aaaabbb"));
    }
}
