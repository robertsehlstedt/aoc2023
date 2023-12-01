fn main() {
    let input = include_str!("./inputs/p2/input.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String{
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./inputs/p2/examplein.txt");
        let output = solution(input);
        let expected = include_str!("./inputs/p2/exampleout.txt");
        assert_eq!(output, expected);
    }
}