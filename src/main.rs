use std::io;

type CalcInt = f64;
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
    const BASE: CalcInt = 10.0;
    let mut value: CalcInt = 0.0;
    loop {
        let digit: CalcInt;
        match token(ps) {
            '0' => digit = 0.0,
            '1' => digit = 1.0,
            '2' => digit = 2.0,
            '3' => digit = 3.0,
            '4' => digit = 4.0,
            '5' => digit = 5.0,
            '6' => digit = 6.0,
            '7' => digit = 7.0,
            '8' => digit = 8.0,
            '9' => digit = 9.0,
            _ => break
        }
        if digit >= BASE {
            bad_formula(format!("Digit {digit} out of range for base {BASE}"));
        }
        if value > (std::f64::MAX - digit)/BASE {
            bad_formula(format!("Float overflow"));
        }
        value = value*BASE+digit;
        ps.index += 1;
    }
    return value
}
/// Digit Parse
/// Operators
fn brackets (ps: &mut ParseState) -> CalcInt {
    let value: CalcInt;
    if token(ps) == '(' {
        lex_match(ps, '(');
        value = add_subtract(ps);
        lex_match(ps, ')');
    }
    else if token(ps).is_digit(10) || token(ps) == '+' || token(ps) == '-' {
        value = scan_digits(ps);
    }
    else {
        value = 0.0;
        bad_formula("Bad Formula".to_string());
    }
    return value
}
fn exponent (ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = brackets(ps);
    let exponent_value: CalcInt = value;
    let mut counter: i64 = 1;
    if token(ps) == '^' {
        lex_match(ps, '^');
        let pow: i64 = brackets(ps).round() as i64;
        if pow == 0 {
            return 1 as f64
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
fn multiply_divide_modulo (ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = exponent(ps);
    while token(ps) == '*' || token(ps) == '/' || token(ps) == '%' {
        match token(ps) {
            '*' => {
                lex_match(ps, '*');
                value *= exponent(ps);
            },
            '/' => {
                lex_match(ps, '/');
                value /= exponent(ps);
            },
            '%' => {
                lex_match(ps, '%');
                let modulo: i64 = exponent(ps).round() as i64;
                if modulo == 0 {
                    return 0 as f64
                } 
                else {
                    value %= modulo as f64
                }
            },
            _ => {},
        }
    }
    return value
}
fn add_subtract (ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = multiply_divide_modulo(ps);
    match token(ps) {
        '+' => {
            lex_match(ps, '+');
            value += multiply_divide_modulo(ps);
        },
        '-' => {
            lex_match(ps, '-');
            value -= multiply_divide_modulo(ps);
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
    match io::stdin().read_line(&mut ps.line) {
        Ok(_n) => {
            let result: CalcInt = add_subtract(&mut ps);
            println!("Result: {result}");
        },
        Err(error) => bad_formula(format!("Error: {error}")),
    }
}