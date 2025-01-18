use std::io;

fn plus(x:i32, y:i32) -> i32 {
    return x + y 
}

fn minus(x:i32, y:i32) -> i32 {
    return x - y 
}

fn multipy(x:i32, y:i32) -> i32 {
    return x * y 
}

fn divide(x:i32, y:i32) -> i32 {
    return x / y 
}

fn power(x:i32, y:i32) -> u32 {
    let num1 = x as u32;
    let num2 = y as u32;
    return (num1.pow(num2));
}

fn main() {
    println!("### Welcome To Rust Calculator ###");
    println!("1: Plus \n2: Minus \n3: Multipy \n4: Divide \n5: Power");
    println!("\n## Pleas Enter a number between 1-5 :");
    let mut user_inp = String::new();
    io::stdin().read_line(&mut user_inp).unwrap();
    
    println!("\n## First Number :");
    let mut inp1 = String::new();
    io::stdin().read_line(&mut inp1).unwrap();
    let num1 = inp1.trim().parse::<i32>().unwrap();
    
    println!("\n## Secend Number :");
    let mut inp2 = String::new();
    io::stdin().read_line(&mut inp2).unwrap();
    let num2 = inp2.trim().parse::<i32>().unwrap();
    
    match user_inp.trim().parse::<i8>().unwrap(){
        1 => {
            println!("\n{} + {} = {}",num1, num2, plus(num1,num2));
        },
        2 => {
            println!("\n{} - {} = {}",num1, num2, minus(num1,num2));
        },
        3 => {
            println!("\n{} x {} = {}",num1, num2, multipy(num1,num2));
        },
        4 => {
            println!("\n{} / {} = {}",num1, num2, divide(num1,num2));
        },
        5 => {
            println!("\n{} ^ {} = {}",num1, num2, power(num1,num2));
        },
        _ => {
            println!("its not on the list");
        }
    };
    
}


