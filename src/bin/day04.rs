use core::panic;

struct AssignmentPair {
    first: (usize, usize),
    second: (usize, usize),
}

impl AssignmentPair {
    fn new(line: &str) -> AssignmentPair {
        let entries: Vec<Vec<usize>> = line
            .split(',')
            .map(|split| {
                split
                    .split('-')
                    .map(|num| match num.parse::<usize>() {
                        Ok(number) => number,
                        Err(err) => {
                            panic!("Unable to parse this: {} because of error: {}", num, err)
                        }
                    })
                    .collect()
            })
            .collect();
        AssignmentPair {
            first: (entries[0][0], entries[0][1]),
            second: (entries[1][0], entries[1][1]),
        }
    }

    fn full_range(&self) -> bool {
        (self.first.0 <= self.second.0 && self.first.1 >= self.second.1)
            || (self.first.0 >= self.second.0 && self.first.1 <= self.second.1)
    }

    fn any_overlap(&self) -> bool {
        self.first.0 <= self.second.1 && self.second.0 <= self.first.1
    }
}

fn solutiona(input: &str) -> usize {
    input
        .split('\n')
        .map(AssignmentPair::new)
        .map(|val| val.full_range())
        .filter(|val| *val)
        .count()
}

fn solutionb(input: &str) -> usize {
    input
        .split('\n')
        .map(AssignmentPair::new)
        .map(|val| val.any_overlap())
        .filter(|val| *val)
        .count()
}

fn main() {
    let start = std::time::Instant::now();
    let text = include_str!("day04.txt");
    println!("Day 4a: {}", solutiona(text));
    println!("Day 4b: {}", solutionb(text));
    println!("Time elapsed: {:?} seconds", start.elapsed())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examplea() {
        let text = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        assert_eq!(solutiona(text), 2);
    }

    #[test]
    fn exampleb() {
        let text = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        assert_eq!(solutionb(text), 4);
    }
}
