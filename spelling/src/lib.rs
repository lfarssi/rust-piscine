pub fn spell(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    
    let mut res = String::new();

    // Handle millions part
    if n >= 1_000_000 {
        res.push_str(&format!("{} million", hundred(n / 1_000_000)));
    }

    // Handle thousands part (ensure it's not added if it's zero)
    if n >= 1000 {
        let thousands_part = (n / 1000) % 1000;
        if !res.is_empty() {
            res.push(' ');
        }
        res.push_str(&format!("{} thousand", hundred(thousands_part)));
    }

    // Handle the remaining hundreds, tens, and ones part
    let remainder = n % 1000;
    if remainder != 0 || n == 0 {  // This is for when the number is not exactly divisible by 1000
        if !res.is_empty() {
            res.push(' ');
        }
        res.push_str(&hundred(remainder));
    }

    res.trim().to_string() // Clean up any unnecessary leading or trailing spaces
}

// Handle ones (1 to 9)
fn ones(n: u64) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "",
    }
}

// Handle teens (10 to 19)
fn teens(n: u64) -> &'static str {
    match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    }
}

// Handle tens (20, 30, 40,..., 90)
fn tens(n: u64) -> &'static str {
    match n {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    }
}

// Handle hundreds (100 to 999)
fn hundred(n: u64) -> String {
    let ones_str = ones(n % 10);  // For ones digit
    let tens_str = tens((n / 10) % 10);  // For tens digit
    let hundreds_str = ones(n / 100);  // For hundreds digit

    let mut result = String::new();

    if n >= 100 {
        result.push_str(&format!("{} hundred", hundreds_str));
    }

    // Add tens and ones if not zero
    if n % 100 != 0 {
        if !result.is_empty() {
            result.push(' ');
        }
        
        // If the number is between 10-19, handle it as a teen
        if n % 100 < 20 {
            result.push_str(teens(n % 100));
        } else {
            result.push_str(tens((n / 10) % 10));  // Add tens (e.g., 20, 30, etc.)
            if n % 10 > 0 {
                result.push('-');
                result.push_str(ones(n % 10));  // Add ones if necessary (e.g., 21, 33, etc.)
            }
        }
    }

    result
}
