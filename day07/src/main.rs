use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq)]
struct BagCount(usize, String);

fn parse_bag_rules(contents: &str) -> HashMap<String, Vec<BagCount>> {
    let rule_pattern = r"([a-z]+ [a-z]+) bags contain (( ?([0-9]+) ([a-z]+ [a-z]+) bags?,?)+).";
    let rule_pattern = Regex::new(rule_pattern).expect("Pattern compile failed");

    let bag_pattern = r"([0-9]+) ([a-z]+ [a-z]+) bags?";
    let bag_pattern = Regex::new(bag_pattern).expect("Pattern compile failed");

    rule_pattern.captures_iter(contents)
        .map(|c| {
            (
                c.get(1).expect("Matches should have capture group 1").as_str().to_string(),
                bag_pattern.captures_iter(c.get(2).expect("Matches should have capture group 2").as_str())
                    .map(|c| {
                        BagCount(
                            c.get(1)
                                .expect("Matches of bag pattern should have capture group 1")
                                .as_str()
                                .parse::<usize>().expect("Capture matching [0-9]+ should parse..."),
                            c.get(2)
                                .expect("Matches of bag pattern should have capture group 2")
                                .as_str()
                                .to_string()
                        )
                    }).collect()
            )
        }).collect()
}

fn count_can_contain(bag_rules: &HashMap<String, Vec<BagCount>>, origin: String) -> usize {
    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();

    to_visit.push_back(origin);

    while let Some(col) = to_visit.pop_front() {
        visited.insert(col.clone());

        for (bag, children) in bag_rules {
            if !visited.contains(bag) && children.iter().any(|b| b.1 == col) {
                to_visit.push_back(bag.clone());
            }
        }
    }

    // Dont count the origin colour
    return visited.len() - 1;
}

fn count_required_inside(bag_rules: &HashMap<String, Vec<BagCount>>, origin: String) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(origin);

    let mut count = 0;

    while let Some(col) = to_visit.pop_front() {
        if let Some(rules) = bag_rules.get(&col) {
            for bag in rules {
                count += bag.0;
                for _ in 0..bag.0 {
                    to_visit.push_back(bag.1.clone());
                }
            }
        }
    }

    return count;
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");
    let bag_rules = parse_bag_rules(&contents);

    println!("Part 1, count = {}", count_can_contain(&bag_rules, "shiny gold".to_string()));
    println!("Part 2, count = {}", count_required_inside(&bag_rules, "shiny gold".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sample = "light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.";

        let bags = parse_bag_rules(&sample);
        
        assert_eq!(bags.get("bright white").cloned(), Some(vec![BagCount(1, "shiny gold".to_string())]));
        assert_eq!(bags.get("vibrant plum").cloned(), Some(vec![
            BagCount(5, "faded blue".to_string()), BagCount(6, "dotted black".to_string())]));

        assert_eq!(count_can_contain(&bags, "shiny gold".to_string()), 4);
    }

    #[test]
    fn test_part2() {
        let sample = "shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.";

        let bags = parse_bag_rules(&sample);
        assert_eq!(count_required_inside(&bags, "shiny gold".to_string()), 126);
    }
}
