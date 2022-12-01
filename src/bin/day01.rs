fn generate_list(input: &str) -> Vec<usize> {
    let mut elf_calories = Vec::new();

    for grouping in input.split("\n\n") {
        let mut count: usize = 0;
        for calorie in grouping.split('\n') {
            count += calorie.parse::<usize>().unwrap();
        }
        elf_calories.push(count);
    }
    elf_calories
}

fn solution(input: &str) -> usize {
    *generate_list(input).iter().max().unwrap()
}

fn solutionb(input: &str) -> usize {
    let mut calories = generate_list(input);
    calories.sort();
    calories = calories.into_iter().rev().collect();
    calories[0] + calories[1] + calories[2]
}

fn main() {
    let text = include_str!("day01.txt");
    println!("Day 1a: {}", solution(text));
    println!("Day 1b: {}", solutionb(text));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let sample_str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(solution(sample_str), 24000)
    }

    #[test]
    fn exampleb() {
        let sample_str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(solutionb(sample_str), 45000)
    }
}
