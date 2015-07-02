extern crate rand;

use std::io;
use rand::Rng;
use std::thread::sleep_ms as sleep;
const YES:&'static str = "Y";
struct Character {
    name: String,
    health: i32,
    attack_rating: i32,
    attack_die: i32,
    defense_die : i32,
    defense_rating : i32
}

impl Character {
    fn take_damage(&mut self, damage:i32){
        self.health -= damage;
    }
}

fn roll_die(min:i32, max:i32) -> i32{
    let roll = rand::thread_rng().gen_range(min, max);
    return roll;
}


fn main() {
    let mut input  = String::new();
    println!("Input your name");
    io::stdin().read_line(&mut input)
    .ok()
    .expect("Failed to read line");
    let mut Player = Character {
        name : input,
        health : 100,
        attack_die : 6,
        attack_rating : 13,
        defense_die : 6,
        defense_rating : 7
    };
    let mut Baddie = Character{
        name : "Boss Dude".to_string(),
        health : 100,
        attack_rating : 8,
        attack_die : 20,
        defense_die : 6,
        defense_rating : 4
    };
    println!("Welcome to battle : {}", Player.name);
    loop {
        let mut attack_roll = 0;

        loop {
            let roll = roll_die(1, Player.attack_roll);
            if(roll == 1){
                println!("Dud! You rolled 1.");
                break;
            }
            attack_roll += roll;
            println!("You rolled a {}, your current power is {}. Roll again?", );
            let mut roll_again = String::new();
            io::stdin().read_line(&mut roll_again)
            .ok()
            .expect("Failed to read line");

            if(roll_again.trim() == YES){
                println!("Rolling!");
            }else{
                break;
            }
        }

        let defense_roll = roll_die(1, Baddie.attack_die);
        let defense_power = defense_roll + Baddie.defense_rating;
        println!("{} rolled a {} with a defense rating of {} it can block {} damage!",
            Baddie.name, defense_roll, Baddie.defense_rating, defense_power );
        let damage_taken = if attack_power - defense_power > 0 {attack_power - defense_power} else {0};
        println!("{} takes {} damage!", Baddie.name, damage_taken);
        Baddie.take_damage(damage_taken);
        if(Baddie.health <= 0){
            return println!("You win!");
        }

        let baddie_attack_roll = roll_die(1, Baddie.attack_die);
        let baddie_attack_power = Baddie.attack_rating + baddie_attack_roll;
        println!("{} rolled {}. With an attack rating of {} it can deal up to : {}", 
            Baddie.name, baddie_attack_roll, Baddie.attack_rating, Baddie.attack_rating + baddie_attack_roll);

        let player_defense_roll = roll_die(1, Player.defense_die);
        let player_defense_power = Player.defense_rating + player_defense_roll;
        println!("You rolled a {} with a defense rating of {} you can blocks {} damage!",
            player_defense_roll, Player.defense_rating, player_defense_power);

        let player_damage_taken = if baddie_attack_power - player_defense_power > 0 {baddie_attack_power - player_defense_power} else {0};
        println!("You take {} damage!", player_damage_taken);
        Player.take_damage(player_damage_taken);
        if(Player.health <= 0){
            return println!("Get Rekt");
        }

        sleep(1000);
    }
}
