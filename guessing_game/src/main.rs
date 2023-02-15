use std::io;
use rand::Rng;
use colored::*;
use std::cmp::Ordering;
fn main() {
   
    let num=rand::thread_rng().gen_range(1,101);
    println!("{}",num);
loop{    
    println!("guess a number");
  
    let mut user_input=String::new();
    io::stdin().read_line(&mut user_input).unwrap();
   
    let mut user_input =match user_input.trim().parse::<i32>(){
        Ok(i)=> i,
        Err(_)=>continue,
    };
    

    println!("user guessed {}",&mut user_input);

    match user_input.cmp(&num){
        Ordering::Less=>println!("{}","too small".purple()),
        Ordering::Equal=>{
            println!("{}","success".green());
            break;

        },
        Ordering::Greater=>println!("{}","too big".purple())
    }

    }

}

