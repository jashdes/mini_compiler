#[derive(Debug, PartialEq)]
enum Token{
    Number(i64),
    Plus,
    Eof, // End of file
}

fn main() {
    let input = "42 + 378";
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if ch == ' ' {
            i += 1;
            continue;
        } else if ch == '+' {
            tokens.push(Token:: Plus);
            i += 1;
        } else if ch.is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            let num_str: String = chars[start..i].iter().collect();
            let num = num_str.parse::<i64>().unwrap();
            tokens.push(Token::Number(num));
        }
    }

    tokens.push(Token::Eof);
    println!("{:?}", tokens);
}
