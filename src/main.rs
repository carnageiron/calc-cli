use std::io::{stdin,stdout, Write};

fn read(input: &mut String){
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}




fn main() {
    println!("Calculator");
    println!("==========");

    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut operator = String::new();
    
    println!("Enter the first number:");
    read(&mut n1);
    
    println!("Enter a second number");
    read(&mut n2);
    
    println!("What operation would you like to perform?[+-*/]");
    read(&mut operator);

    let n1: f32 = n1.trim().parse().unwrap();
    let n2: f32 = n2.trim().parse().unwrap();
    let operator: char = operator.trim().chars().next().unwrap();

    let operators = String::from("+-*/");

    if !operators.contains(operator){
        println!("unknown operator");
        return;
    }

    let result = match operator{
        '+' => n1 + n2,
        '-' => n1 - n2,
        '*' => n1 * n2,
        '/' => n1 / n2,
        _ => panic!("error in operator")
    };

    println!("the result is {}", result)
}


