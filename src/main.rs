use std::io;

type CalcInt = i64;
/// Parse, Token Index and Errors
struct ParseState {
    line: String,
    index: usize,
}
fn token (ps: &ParseState) -> char {
    ps.line.as_bytes()[ps.index-1] as char
}
fn bad_formula (msg: String) {
    eprintln!("Bad Formula: {msg}");
    ::std::process::exit(1);
}
fn lex_match (ps: &mut ParseState, expected: char) {
    if token(ps) == expected {
        ps.index += 1;
        return
    }
    bad_formula(format!("Error Matching {expected} at index {}", ps.index));
    std::process::exit(1);    
}
/// Parse, Token Index and Errors
/// Main
fn main() {
    println!("\nBrackets\t()\nAddition\t+\nSubsraction\t-\nMultiplication\t*\nDivision\t/\nExponentiation\t^\nModulus\t\t%\n");
    println!("Sample Question: 2+4*8/(8%(2^3))\n");

    let mut ps = ParseState {
        line: String::new(),
        index: 1,
    };
    print!("Expression: \n\t");
}