
use std::io;
fn getInput(){

}


fn main() {
    println!("Welcome to Temple,s Calculator");
    println!("Enter Operand1");
    let mut operand1 = String::new();
    io::stdin()
        .read_line(&mut operand1)
        .expect("Failed to read line");
    let first_number: u32 = operand1.trim().parse().expect("Wanted a number");
    println!("Enter Operand2");
    let mut operand2 = String::new();
    io::stdin()
        .read_line(&mut operand2)
        .expect("Failed to read line");
   
    let second_number: u32 = operand2.trim().parse().expect("Wanted a number");
    if first_number<second_number {
        println!("{} is less than {}",first_number,second_number);
        println!("This operation cant go through");
        
    }
    let  add_result = first_number + second_number;
    let  mul_result:u32 = first_number * second_number;
    let  div_result:u32 = first_number / second_number;
    let  sub_result:u32 = first_number - second_number;
    
    println!("Addtion:{} + {} = {}",first_number,second_number,add_result);
    println!("Multiplication:{} * {} = {}",first_number,second_number,mul_result);
    println!("Division:{} / {} = {}",first_number,second_number,div_result);
    println!("Subtraction:{} - {} = {}",first_number,second_number,sub_result);
    
    
    
}  
