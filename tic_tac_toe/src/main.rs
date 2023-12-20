 
use std::io;
use rand::Rng;
 
// MY FIRST GAME IN RUST
fn main() {
    let choices: [&str; 3]=["Rock", "Paper", "Scissor"];
    println!("SAMPLE TIC TAC TOE GAME");
    get_user_choice();
    get_comp_choice();
}

// Function to get computer choice
fn get_comp_choice(){
    let rand_number=rand::thread_rng().gen_range(1..=3);
    println!("Computer range is {}",rand_number);
    let comp: &str="";

}
// Function to get user choice
fn get_user_choice(){
    let mut user_choice: String=String::new();
    println!("Enter either Rock, paper or scissor");

    io::stdin()
    .read_line(&mut user_choice)
    .expect("Erro getting user input");

   println!("Your choice is {}", user_choice)
 
}

// Function to get the winner
fn find_winner(){
    
    
  
}

// The overal Function
fn playground(){

}