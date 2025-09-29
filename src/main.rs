use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut operator = String::new();

    loop {
        println!("First number\n");

        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");
        let input1: u32 = match input1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Second number\n");
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line");
        let input2: u32 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Operator\n1 = + | 2 = - | 3 = * | 4 = /\n");
        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read line");
        let operator: i32 = match operator.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match operator{
            1=> { let sum = input1 + input2; println!("your sum is: {sum}")},
            2=> { let difference = input1 - input2; println!("your difference is: {difference}")},
            3=> { let product = input1 * input2; println!("your product is: {product}")},
            4=> { let quotient = input1 / input2; println!("your quotient is: {quotient}")},
            _=>println!("Invalid"),
        };
    }
}
