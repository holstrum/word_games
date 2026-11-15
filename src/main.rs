use  std::io;
use rand::Rng;
fn main() {

    println!("
    WELCOME TO HOLSTRUMS GAMING WORLD----
    
    NOW THEN ENTER YOUR GAMER NAME  TO PROCEED");

    let mut  name = String::new();

    io::stdin()
    .read_line(& mut name)
    .expect("failed to read line");

    let input : & str = name
    .trim();

    println!("
    WELL HEY THERE {input}-----
    READY FOY THE GAME OF YOUR LIFE 
    >>>YES
    >>>NO");

    let  mut gaming_continue = String::new();

    io::stdin()
    .read_line(& mut gaming_continue)
    .expect("failed to read line");

    let input_game : & str = gaming_continue
    .trim();

    if input_game == "yes"
    { 

    

    println!("
    
    PLEASE SECLECT A GAME TO PROCEED-----
    
    
    1:Russian roullette
    2: Tick Tack Toe"
);

    let mut game_select = String::new();

    io::stdin()
    .read_line(& mut game_select)
    .expect("failed to read line");

    let input : usize = game_select
    .trim()
    .parse()
    .expect("please enter an integer");

    if input == 1
    {
    russian(& name);

    }
 






}
else {
    println!("
    EXITING GAME ************ ");
    return;
}
}
fn russian (x:& str){
    println!("
    Well you seem like a reasonable person -{x}
    you wouldnt mind risking your life now would you ---


    1: I mean its alright
    2: I do mind
    ");

    let  mut risk = String::new();

    io::stdin().read_line(& mut risk).expect("read failed");

    let risk_in : usize = risk
    .trim()
    .parse()
    .expect("not a number");

    if risk_in == 1
    {
        println!(" Thats more like it I knew you had guts")
    }

    println!("
    HERE IS THE GAME-----
    THERE IS A REVOLVER ON THE TABLE ---
    WE WILL TAKE TURNS TO SPIN THE BARREL --
    AND FIRE AT EACH OTHER IF THE BULLET LANDS IN THE CAMBER ---
    THE PERSON FIRED AT DIES-- AND LOOSES

    NOW  then who should go first 

    >>> ME OR YOU 

    ");

    let mut first = String::new();

    io::stdin()
    .read_line(&mut first )
    .expect("dang i cant get whose first");

    let first_run = first.trim();

    if first_run == "me" {

        player_starts();
    }

    else {
        println!("
        WELL --- WELL --- WELL
        BRACE FOR IMPACT------");

        computer_starts();
    }



}

fn player_starts(){


        println!("
        OKAY ---- TAKE THE GUN
        SPIN THE BARELL -------
        WRITE --fire-- TO FIRE THE GUN ");

    let mut fire = String::new();

    io::stdin().read_line(& mut fire). expect("failed to fire gun");

    let  fired_gun = fire.trim();

    if  fired_gun == "fire"{

    let sectret_1 = rand::thread_rng().gen_range(1..= 6);


    println!("secret number 1 is {sectret_1}");
    let y :i32 = 1;
    if sectret_1 == y {

        println!("
        BOOM----
        TARGET ELIMINATED ---
        YOU GOT SUM LUCK THERE YOU JUST WON*");

        return;
    }

    else {
        
        println!("
        CLICK*-----
        NOW ITS MY TURN ");
        computer_starts();
    }

}
}
fn computer_starts(){

        println!("
        BARRELL SPIN-----
        ARE YOU READY--
        >>YES
        >>NO ");

    let mut ready = String::new();
    io::stdin().read_line(& mut ready).expect("i cant tell if youre ready or not");

    let player_ready = ready.trim();

    if player_ready == "yes"{

      let sectret_2 = rand::thread_rng().gen_range(1..=6);

      println!("secret number is 2 {sectret_2}");
      let z:i32 = 1;

    if sectret_2 == z{

        println!("
        BOOM----
        **********************
        **********************
        UNFORTUNATELY YOU DIED");

        return;
    }

    else {

        println!("
        click **
        OH WELL I GUESS ITS YOUR TURN");

        player_starts();
    }


}
}