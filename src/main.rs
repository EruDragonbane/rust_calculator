use std::io;

type CalcFloat = f64;
// Parse, Token Index and Errors
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
// Parse, Token Index and Errors
// Digit Parse
fn scan_digits (ps: &mut ParseState) -> CalcFloat {
    const BASE: CalcFloat = 10.0;
    let mut value: CalcFloat = 0.0;
    loop {
        let digit: CalcFloat;
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
// Digit Parse
// Operators
fn brackets (ps: &mut ParseState) -> CalcFloat {
    // Soooooo, the problem is here
    // https://media.makeameme.org/created/i-mean-i-cb78e603c1.jpg 
    // If the operation is an addition or subtraction, the program stops.
    let value: CalcFloat;
    if token(ps) == '(' {
        lex_match(ps, '(');
        value = add_subtract(ps);
        lex_match(ps, ')');
    }
    else if token(ps).is_digit(10) == true && token(ps) != '+' {
        value = scan_digits(ps);
    }
    else {
        value = 0.0;
        bad_formula("Bad Formula".to_string());
    }
    return value
}
fn factorial (ps: &mut ParseState) -> CalcFloat {
    let mut value: i64 = brackets(ps) as i64;
    print!("Value: {value}");
    if token(ps) == '!' {
        let mut factorial: i64 = 1;
        let mut counter: i64 = 1;
        if value <= 1{
            return 1 as f64
        }
        else {
            while counter <= value {
                factorial *= counter;
                counter += 1;
            }
            value = factorial;
        }
    }
    return value as f64;
}
fn float_dot (ps: &mut ParseState) -> CalcFloat {
    let mut value: String = factorial(ps).to_string();
    if token(ps) == '.' {
        value = value + ".";
        lex_match(ps, '.');     
        let a = brackets(ps).to_string();
        value.push_str(&a);
        return value.parse().unwrap()
    }
    else {
        return value.parse().unwrap()
    }
}
fn exponent (ps: &mut ParseState) -> CalcFloat {
    let mut value: CalcFloat = float_dot(ps);
    if token(ps) == '^' {
        let exponent_value: CalcFloat = value;
        let mut counter: i64 = 1;
        lex_match(ps, '^');
        let pow: i64 = float_dot(ps).round() as i64;
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
fn multiply_divide_modulo (ps: &mut ParseState) -> CalcFloat {
    let mut value: CalcFloat = exponent(ps);
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
fn add_subtract (ps: &mut ParseState) -> CalcFloat {
    let mut value: CalcFloat = multiply_divide_modulo(ps);
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
// Operators
// Main
fn main() {
    println!("
    Brackets\t\t()
    Addition\t\t+
    Subtraction\t\t-
    Multiplication\t*
    Division\t\t/
    Exponentiation\t^
    Modulus\t\t%
    Float\t\t.
    Factorial\t\t!");
    println!("Sample Question: 2+4*8/(8/(2^3))\n");

    let mut ps = ParseState {
        line: String::new(),
        index: 1,
    };
    print!("Expression: \n\t");
    match io::stdin().read_line(&mut ps.line) {
        Ok(_n) => {
            let result: CalcFloat = add_subtract(&mut ps);
            if result == result.round() {
                println!("Result: {result}");
            }
            else {
                println!("Result: {:.2}", result);
            }
        },
        Err(error) => bad_formula(format!("Error: {error}")),
    }
}