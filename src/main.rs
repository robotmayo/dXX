extern crate rand;

use std::io;
use rand::Rng;
use std::thread::sleep_ms as sleep;
const YES:&'static str = "Y";
struct Character {
    name: String,
    health: i32,
    attack_die: i32,
    defense_die : i32,
    defense_rating : i32
}

impl Character {
    fn take_damage(&mut self, damage:i32){
        self.health -= damage - self.defense_rating;
        println!("{} took {} damage!", self.name, damage - self.defense_rating);
        println!("{} has {} health remaining!", self.name, self.health);
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
        defense_die : 6,
        defense_rating : 7
    };
    let mut Baddie = Character{
        name : "Boss Dude".to_string(),
        health : 100,
        attack_die : 20,
        defense_die : 6,
        defense_rating : 4
    };
    println!("Welcome to battle : {}", Player.name);
    loop {
        let mut player_total_attack_roll = 0;
        let mut player_num_dice_rolled = 0;

        loop {
            let roll = roll_die(1, Player.attack_die);
            player_num_dice_rolled += 1;
            if roll == 1 {
                println!("Dud! You rolled 1.");
                player_total_attack_roll = 1;
                break;
            }
            player_total_attack_roll += roll;
            println!("You rolled a {}, your current power is {}. Roll again?", roll, player_total_attack_roll);
            let mut roll_again = String::new();
            io::stdin().read_line(&mut roll_again)
            .ok()
            .expect("Failed to read line");

            if roll_again.trim() == YES {
                println!("Rolling!");
            }else{
                break;
            }
        }
        println!("You rolled {} dice for a total of {}", player_num_dice_rolled, player_total_attack_roll);
        Baddie.take_damage(player_total_attack_roll);

        let baddie_max_rolls = 5;
        let mut baddie_num_dice_rolled = 0;
        loop {
            let roll = roll_die(1, Baddie.attack_die);
        }

        sleep(1000);
    }
}
