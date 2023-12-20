 
use std::io;
use rand::Rng;
 
// MY FIRST GAME IN RUST
fn main() {
     println!("SAMPLE TIC TAC TOE GAME");
    let us_choice: &str= get_user_choice();
    let com_choice: &str= get_comp_choice();
    let get_winner: &str=find_winner(us_choice, com_choice);
    println!("Winner is {}", get_winner);
 
}

// Function to get computer choice
fn get_comp_choice() ->& 'static str{

    let choices: [&str; 3]=["Rock", "Paper", "Scissor"];
    let rand_number: usize=rand::thread_rng().gen_range(1..=3);
     let comp_choice: &str=choices[rand_number];
    println!("Computer choice is {}",comp_choice);
    comp_choice


 
}
// Function to get user choice
fn get_user_choice() ->& 'static str{
    let mut user_choice=String::new();
    println!("Enter either Rock, paper or scissor");
    io::stdin()
    .read_line(&mut user_choice)
    .expect("Erro getting user input");
    
   println!("Your choice is {}", user_choice.trim());
   
    match user_choice.trim().to_lowercase().as_str() {
        "rock" => "Rock",
        "paper" => "Paper",
        "scissor" | "scissors" => "Scissor",
        _ => panic!("Invalid choice"), // Handle invalid input
    }

    
  
}

// Function to get the winner
fn find_winner(user:&str, computer:&str)->& 'static str{
    
    if user == computer {
        "draw"

    }
    else if (computer == "Paper" && user == "Rock") ||
            (computer == "Rock" && user == "Scissors") ||
            (computer == "Scissors" && user == "Paper")
     {
        "computer  wins"
    }
    else {
      "user wins"
   }
  
}
    

 