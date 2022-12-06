struct RuckSack<'a> {
    items: Vec<&'a str>,
    // first: &'a str,
    // second: &'a str,
}

impl RuckSack<'_> {
    fn new<'a>(items: Vec<&'a str>) -> RuckSack<'a> {
        RuckSack { items }
    }

    fn both(&self) -> char {
        let mut first_chars: Vec<char> = self.items[0].chars().collect();
        for item in &self.items {
            first_chars = first_chars
                .into_iter()
                .filter(|val| item.contains(*val))
                .collect();
        }
        first_chars[0]
        // let second_chars: Vec<char> = self.second.chars().collect();
        // self.first
        //     .chars()
        //     .into_iter()
        //     .filter(|val| second_chars.contains(val))
        //     .next()
        //     .unwrap()
    }

    fn both_priority(&self) -> usize {
        let both = self.both();
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .find(both)
            .unwrap()
            + 1
    }
}

fn solutiona(input: &str) -> usize {
    let mut real_rucksacks = Vec::new();
    for row in input.split('\n') {
        let (first, second) = row.split_at(row.len() / 2);
        real_rucksacks.push(RuckSack::new(vec![first, second]));
    }
    real_rucksacks.iter().map(|r| r.both_priority()).sum()
}

fn solutionb(input: &str) -> usize {
    let mut group = Vec::new();
    let mut groups = Vec::new();
    for row in input.split('\n') {
        group.push(row);
        if group.len() == 3 {
            groups.push(RuckSack::new(group.clone()));
            group.clear();
        }
    }
    groups.iter().map(|val| val.both_priority()).sum()
}

fn main() {
    let text = include_str!("day03.txt");
    println!("Day 3a: {}", solutiona(text));
    println!("Day 3b: {}", solutionb(text));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examplea() {
        let text = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solutiona(text), 157);
    }

    #[test]
    fn exampleb() {
        let text = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solutionb(text), 70)
    }
}
