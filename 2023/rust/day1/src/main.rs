fn main() {
    println!("Hello, world!");
    let mut input = include_str!("../d1p1.txt");
    println!("{}", d1p1(input));
    input = include_str!("../d1p2.txt");
    println!("{}", d1p2(input));
}

fn d1p1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let first_digit: i32 = line
            .chars()
            .filter(|x| x.is_digit(10))
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>()[0];
        let last_digit: i32 = line
            .chars()
            .rev()
            .filter(|x| x.is_digit(10))
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>()[0];

        total += first_digit * 10 + last_digit;
    }
    return total;
}

fn d1p2(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let mut first_digit = -1;
        for i in 0..line.len() {
            first_digit = match line {
                _ if line[i..].starts_with("one") => 1,
                _ if line[i..].starts_with("two") => 2,
                _ if line[i..].starts_with("three") => 3,
                _ if line[i..].starts_with("four") => 4,
                _ if line[i..].starts_with("five") => 5,
                _ if line[i..].starts_with("six") => 6,
                _ if line[i..].starts_with("seven") => 7,
                _ if line[i..].starts_with("eight") => 8,
                _ if line[i..].starts_with("nine") => 9,
                _ if line[i..].starts_with("1") => 1,
                _ if line[i..].starts_with("2") => 2,
                _ if line[i..].starts_with("3") => 3,
                _ if line[i..].starts_with("4") => 4,
                _ if line[i..].starts_with("5") => 5,
                _ if line[i..].starts_with("6") => 6,
                _ if line[i..].starts_with("7") => 7,
                _ if line[i..].starts_with("8") => 8,
                _ if line[i..].starts_with("9") => 9,
                _ => -1,
            };
            if first_digit != -1 {
                break;
            }
        }
        let mut last_digit = -1;
        for i in 0..line.len() {
            last_digit = match line {
                _ if line[..line.len() - i].ends_with("one") => 1,
                _ if line[..line.len() - i].ends_with("two") => 2,
                _ if line[..line.len() - i].ends_with("three") => 3,
                _ if line[..line.len() - i].ends_with("four") => 4,
                _ if line[..line.len() - i].ends_with("five") => 5,
                _ if line[..line.len() - i].ends_with("six") => 6,
                _ if line[..line.len() - i].ends_with("seven") => 7,
                _ if line[..line.len() - i].ends_with("eight") => 8,
                _ if line[..line.len() - i].ends_with("nine") => 9,
                _ if line[..line.len() - i].ends_with("1") => 1,
                _ if line[..line.len() - i].ends_with("2") => 2,
                _ if line[..line.len() - i].ends_with("3") => 3,
                _ if line[..line.len() - i].ends_with("4") => 4,
                _ if line[..line.len() - i].ends_with("5") => 5,
                _ if line[..line.len() - i].ends_with("6") => 6,
                _ if line[..line.len() - i].ends_with("7") => 7,
                _ if line[..line.len() - i].ends_with("8") => 8,
                _ if line[..line.len() - i].ends_with("9") => 9,
                _ => -1,
            };
            if last_digit != -1 {
                break;
            }
        }
        println!("{} {}{}", line, first_digit, last_digit);
        total += 10 * first_digit + last_digit;
    }
    return total;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../test_input1.txt");
        assert!(d1p1(input) == 142);
    }

    #[test]
    fn test_pt2() {
        let input = include_str!("../test_input2.txt");
        assert!(d1p2(input) == 281);
    }
}
