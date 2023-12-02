fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let output: u32 = input
        .lines()
        .map(|line: &str| {
            let mut it =
                line.chars().filter_map(|c| {c.to_digit(10)
                });
            let first =
                it.next().expect("should be a number");

            match it.last(){
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a number")
        })
        .sum::<u32>();
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, 142);
    }
}