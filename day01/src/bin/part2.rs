fn main() {
    let input = include_str!("./inputs/p2/input.txt");
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
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };
        
        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

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
        let input = include_str!("./inputs/p2/examplein.txt");
        let output = solution(input);
        let expected = include_str!("./inputs/p2/exampleout.txt");
        assert_eq!(output, expected);
    }
}