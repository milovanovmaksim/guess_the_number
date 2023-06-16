use std::io;

use crate::{attempts::Attempts, user::{UserState, UserAnswer}};

pub struct Game {
    attempts: Attempts,
}

impl Game {
    ///
    pub fn new(attempts: Attempts) -> Self {
        Self {
            attempts, 
        }
    }
    ///
    pub fn run(&mut self) {
        println!("Отгадайте число от 0 до 100, которое загодал компьютер.");
        let repeat = true;
        while repeat {
            match self.attempts.run() {
                UserState::GameOver => {
                    let secret = self.attempts.secret();
                    println!("Вы проиграли. Загаданное число - {}", secret);
                }
                UserState::Win => {
                    println!("Вы выиграли!");
                }
                UserState::Quite => {
                    break;
                }
                UserState::InGame => { continue; }
            }
            match self.ask_user_for_restarting_game() {
                UserAnswer::Yes => {
                    return self.run();
                }
                _ => { break; }
            }
        }
        println!("Вы покинули игру.");
    }

    fn ask_user_for_restarting_game(&self) -> UserAnswer {
        println!("\nХотите еще партейку? Если да нажмите \"y\", иначе любую клавишу aA - zZ.");
        let mut answer = String::new();
        match io::stdin().read_line(&mut answer) {
            Ok(_) => {
                if answer.trim().to_lowercase() == "y" {
                    UserAnswer::Yes
                } else {
                    UserAnswer::No
                }
            }
            Err(_) => UserAnswer::No,
        }
    }
}
