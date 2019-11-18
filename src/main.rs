fn main() {
    let isvowl = vowels_finder('x');
    println!("{}", isvowl);
    basic_calculator(23, 32, '+');
}

fn vowels_finder(letter: char) -> bool {
    if letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u' {
        true
    } else {
        false
    }
}

fn basic_calculator(a: i32, b: i32, oper: char) {
    let result = match oper {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => a + b,
    };
    println!("{}", result);
}
