use std::io;
fn main() {
    let mut input = String::new();
    let mut trial=0;
    loop{
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin().read_line(&mut input).expect("Failure");
        trial +=  1;
        if input.trim() == "The letter e"{
            println!("the number of trials : {}", trial);
            break;
        }
    }
}
