fn main() {
    let input = include_str!("./inputs/p1/input.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let input = include_str!("./inputs/p1/examplein.txt");
        let output = solution(input);
        let expected = include_str!("./inputs/p1/exampleout.txt");
        assert_eq!(output, expected);
    }
}