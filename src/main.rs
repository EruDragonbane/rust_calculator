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
/// Digit Parse
fn scan_digits (ps: &mut ParseState) -> CalcInt {
    const BASE: CalcInt = 10;
    let mut value: CalcInt = 0;
    loop {
        let digit: CalcInt;
        match token(ps) {
            '0' => digit = 0,
            '1' => digit = 1,
            '2' => digit = 2,
            '3' => digit = 3,
            '4' => digit = 4,
            '5' => digit = 5,
            '6' => digit = 6,
            '7' => digit = 7,
            '8' => digit = 8,
            '9' => digit = 9,
            _ => break
        }
        if digit >= BASE {
            bad_formula(format!("Digit {digit} out of range for base {BASE}"));
        }
        if value > (std::i64::MAX - digit)/BASE {
            bad_formula(format!("Integer overflow"));
        }
        value = value*BASE+digit;
        ps.index += 1;
    }
    return value
}
/// Digit Parse
///  Operators
fn exponent (ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = brackets(ps);
    let exponent_value: CalcInt = value;
    let mut counter: CalcInt = 1;
    if token(ps) == '^' {
        lex_match(ps, '^');
        let pow: CalcInt = brackets(ps);
        if pow == 0 {
            return 1
        }
        else if pow == 1 {
            return value
        }
        else {
            while counter < pow {
                value *= exponent_value;
                counter += 1;
            }
        }
    }
    return value
}
fn multiply_divide (ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = exponent(ps);
    while token(ps) == '*' || token(ps) == '/' {
        match token(ps) {
            '*' => {
                lex_match(ps, '*');
                value *= exponent(ps);
            },
            '/' => {
                lex_match(ps, '/');
                value /= exponent(ps);
            },
            _ => {},
        }
    }
    return value
}
fn add_subtract (ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = multiply_divide(ps);
    match token(ps) {
        '+' => {
            lex_match(ps, '+');
            value += multiply_divide(ps);
        },
        '-' => {
            lex_match(ps, '-');
            value -= multiply_divide(ps);
        },
        _ => {},
    }
    return value
}
/// Main
fn main() {
    println!("\nBrackets\t()\nAddition\t+\nSubtraction\t-\nMultiplication\t*\nDivision\t/\nExponentiation\t^\nModulus\t\t%\n");
    println!("Sample Question: 2+4*8/(8%(2^3))\n");

    let mut ps = ParseState {
        line: String::new(),
        index: 1,
    };
    print!("Expression: \n\t");
}