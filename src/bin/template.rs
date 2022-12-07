#![allow(unused_variables)]
fn solutiona(input: &str) -> usize {
    0
}

fn solutionb(input: &str) -> usize {
    0
}

fn main() {
    let text = include_str!("dayxx.txt");
    println!("Day xa: {}", solutiona(text));
    println!("Day xb: {}", solutionb(text));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examplea() {
        let text = "";
        assert_eq!(solutiona(text), 0);
    }

    #[test]
    fn exampleb() {
        let text = "";
        assert_eq!(solutionb(text), 0)
    }
}
