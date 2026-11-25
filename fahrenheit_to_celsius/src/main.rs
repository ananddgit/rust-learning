use std::io;

fn main() {
    println!("{:>28}","Fahrenheit To Celsius");
    println!("");

    loop{

        println!("Please enter Fahrenheit: ");

        let mut temperature: String = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("Celsius: {:.2} C", fahrenheit_to_celsius(temperature));
        break;


    }
    
}

fn fahrenheit_to_celsius(f: f64) -> f64 {

    (f - 32.0) * 5.0/9.0

}