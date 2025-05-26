#derive[(Debug, PartialEq)]
enum Token{
    Number(i64),
    Plus,
    Eof, // End of file
}

fn main() {
    let input = "5 + 3";
    println!("Input: {}", input);
}
