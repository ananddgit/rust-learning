use std::io;

fn main() {
    
    loop{    
        println!("Please enter number for fibonacci");

        let mut n: String = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed read line");

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        
        if n > 93 {
            println!("Too large, max is 93.");
            continue;
        }    
        
        println!("Fibonacci number is: {}", fibonacci(n));
        break;
    }
}


fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
       return n;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let c: u64 = a + b;
        a = b;
        b = c;
        }
    return b;
}