mod token;
mod lexer;

fn main() {
    let chars = "Hello, world!".chars().collect::<Vec<char>>();
    let sl: String = chars.get(0..3).unwrap().iter().collect();
    
    dbg!(sl);
}
