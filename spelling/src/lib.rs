pub fn spell(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    let mut res = String::new();

    if n >= 1_000_000 {
        res.push_str(&format!("{} million", hundred(n / 1_000_000)));
    }
    if n >= 1000 {
        if !res.is_empty() {
            res.push(' '); 
        }
        res.push_str(&format!("{} thousand", hundred((n / 1000) % 1000)));
    }
    if n % 1000 != 0 || n == 0 {
        if !res.is_empty() {
            res.push(' ');
        }
        res.push_str(&hundred(n % 1000));
    }
    
    res.trim().to_string() 
}

fn ones(n:u64)-> &'static str{
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

fn hundred(n: u64) -> String {
    let ones_str = ones(n % 10);
    let tens_str = tens((n / 10) % 10);
    let hundreds_str = ones(n / 100);

    let mut result = String::new();

    if n >= 100 {
        result.push_str(&format!("{} hundred", hundreds_str));
    }
    if (n % 100) > 0 {
        if !result.is_empty() {
            result.push(' ');
        }
        if (n % 100) < 10 {
            result.push_str(&ones_str);
        } else if (n % 100) < 20 {
            result.push_str(&teens(n % 100));
        } else {
            result.push_str(&tens((n % 100) / 10));
            if (n % 10) > 0 {
                result.push('-');
                result.push_str(&ones(n % 10));
            }
        }
    }
    result
}
