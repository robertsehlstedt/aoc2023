fn main() {
    let input = include_str!("./inputs/p1/input.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> String{
    input
    .lines()
    .map(process_line)
    .sum::<u32>()
    .to_string()
}

fn process_line(line: &str) -> u32 {
    let mut it =
    line.chars().filter_map(|character| {
        character.to_digit(10)
    });
    let first =
        it.next().expect("should be a number");

    match it.last() {
        Some(num)   => format!("{first}{num}"),
        None        => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./inputs/p1/examplein.txt");
        let output = solution(input);
        let expected = include_str!("./inputs/p1/exampleout.txt");
        assert_eq!(output, expected);
    }
}