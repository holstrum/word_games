use std::io;
fn main() {
loop{
println!("please select the game mode below:------------
1: Guessing game
2: Sudoku
3: Tik TAk Toe
4: Dice Roll
5: Game of chance
6: lost at sea game
7: Russina roullette
8: Rock Paper Scissors


");

let mut choice = String::new();

io::stdin()
.read_line(& mut choice)
.expect("failed to read line");

let select:usize = choice
.trim()
.parse()
.expect("invalid choice");

if select == 1{
    guessing_game()
}
else if select == 2 {
    sudoku()
}
else if select == 3 {
    tik_tack()
}else if select == 4 {
    dice_roll()
}else if select == 5 {
    chanced_up()
}else if select == 6{
    lost_at_sea()

}
else {
    russian_roullette();
}
} 


    
/*  guessing_game();
    sudoku();
    tik_tak_toe();
    roll_dice();
    chance_game(); */ 
    
}

fn guessing_game(){
    println!("running guessing game-----");
}

fn sudoku(){
    println!("running sudoku------");
}

fn tik_tack(){
println!("running tik_tack ------");
}

fn dice_roll(){
println!("running dice_roll ------");
}

fn  chanced_up(){
println!("running chanced_up ------");
}

fn  lost_at_sea(){
println!("running lost_at_sea ------");
}

fn russian_roullette(){
    println!("
    
    running russian_roullette -------------
    
    Welcome player to a game of russian roullete
    NOTE : if you loose you die ----
    But thats not something to discourage you
    
    
    so whats your name?

    --- enter name below ---
    
    ");

    let mut name = String::new();

    io::stdin()
    .read_line(& mut name)
    .expect(" failed to fetch name");

let entry_name = name.trim();

println!(" ---------
ahhhhh {entry_name} you sound like a responsible person ---

shall we proceed (YES / NO)

");

let mut  proceed = String::new();

io::stdin()
    .read_line(& mut proceed)
    .expect(" failed to fetch name");

let choice2: &str = proceed.trim();

if choice2 == "yes" {
println!("what game mode would you like
A.constant probability
B.changing probability
")
}

else {

    println!("---- Exiting program -----");
    return;
}

let mut game_mode = String::new();

io::stdin()
    .read_line(& mut game_mode)
    .expect(" failed to fetch name");

let choice3: &str = &game_mode.trim();

if choice3 == "A"{
constant_prob(entry_name);
}

else {
    changing_prob(entry_name);
}
}

fn constant_prob(x:&str){

    println!("Time to play for your life {x}----

    HOW TO PLAY-----
    1: There's a revolver on the table ---
    2: We both take to spin the barrel and fire at each other ---
    3: When the bullet lands in the chamber and fired the opponent dies and effectively looses
    
    ");

}

fn changing_prob(x:&str){

    println!("Time to play for your life {x}");
}