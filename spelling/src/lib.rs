pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut res = String::new();

    if n >= 1_000_000 {
        // let millions_part = n / 1_000_000;
        res.push_str(&format!("one million"));
    }

    if n >= 1000 {
        let thousands_part = (n / 1000) % 1000;
        if !res.is_empty() {
            res.push(' '); 
        }
        res.push_str(&format!("{} thousand", hundred(thousands_part)));
    }

    let remainder = n % 1000;
    if remainder != 0 {
        if !res.is_empty() {
            res.push(' ');
        }
        res.push_str(&hundred(remainder));
    }

    res.trim().to_string()
}

fn hundred(n: u64) -> String {
    if n == 0 {
        return "".to_string();
    }

    let ones_str = ones(n % 10);
    let tens_str = tens((n / 10) % 10);
    let hundreds_str = ones(n / 100);

    let mut result = String::new();

    if n >= 100 {
        result.push_str(&format!("{} hundred", hundreds_str));
    }

    if n % 100 != 0 {
        if !result.is_empty() {
            result.push(' ');
        }
        if n % 100 < 10 {
            result.push_str(&ones_str); 
        } else if n % 100 < 20 {
            result.push_str(&teens(n % 100)); 
        } else {
            result.push_str(&tens_str); 
            if n % 10 > 0 {
                result.push('-');
                result.push_str(&ones(n % 10)); 
            }
        }
    }

    result
}

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
