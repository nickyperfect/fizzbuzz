fn main() {
    for fizzy in 1..101{
        match fizzy {
            _ if fizzy % 15 == 0 => println!("FIZZ-BUZZ"), 
            _ if fizzy % 3 == 0 => println!("FIZZ"), 
            _ if fizzy % 5 == 0 => println!("BUZZ"), 
            _ => println!("{}", fizzy),
        }
    }
}
