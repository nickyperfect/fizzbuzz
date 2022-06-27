fn main() {
    for fizzy in 1..100{
        match fizzy {
            fizzy if (fizzy %15 == 0) => println!("{:?}", "FIZZ-BUZZ"), 
            fizzy if (fizzy %3 == 0) => println!("{:?}", "FIZZ"), 
            fizzy if (fizzy %5 == 0) => println!("{:?}", "BUZZ"), 
            _ => println!("{:?}", fizzy)
        }
    }
}
