#[derive(Debug, PartialEq)]
enum Token{
    Number(i64),
    Plus,
    Eof, // End of file
}

fn main() {
    let input = "5 + 3";
    let mut tokens = Vec::new();

    for ch in input.chars() {
        if ch == ' ' {
            continue;
        } else if ch == '+' {
            tokens.push(Token::Plus);
        } else if ch.is_ascii_digit(){
            let digit = ch.to_digit(10).unwrap() as i64;
            tokens.push(Token::Number(digit));
            }
        }

    tokens.push(Token::Eof);
    println!("{:?}", tokens);
}
