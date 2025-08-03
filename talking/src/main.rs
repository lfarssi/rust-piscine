use talking::*;

fn main() {
    println!("{:?}", talking("JUST DO IT!"));
    println!("{:?}", talking("Hello how are you?"));
    println!("{:?}", talking("WHAT'S GOING ON?"));
    println!("{:?}", talking("something"));
    println!("{:?}", talking(""));
}



#[test]
fn test_yell() {
    assert_eq!(
        talking("JUST DO IT!"),
        "There is no need to yell, calm down!"
    );
    assert_eq!(
        talking("1, 2, 3 GO!"),
        "There is no need to yell, calm down!"
    );
    assert_eq!(
        talking("I LOVE YELLING"),
        "There is no need to yell, calm down!"
    );
    assert_eq!(
        talking("WJDAGSAF ASVF EVA VA"),
        "There is no need to yell, calm down!"
    );
}