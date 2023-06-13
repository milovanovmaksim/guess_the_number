use std::cmp::Ordering;

use crate::{
    user::{User, UserAnswer, UserState},
    computer::Computer,
};


pub struct VerboseDiff {
    user: User,
    computer: Computer,
}

impl VerboseDiff {
    ///
    pub fn new(user: User, computer: Computer) -> Self {
        Self { 
            user: user,
            computer: computer,
        }
    }
    ///
    pub fn restart(&mut self) {
        self.computer.restart();
    }
    /// 
    fn compare(&mut self) -> Option<Ordering> {
        match self.user.guess() {
            UserAnswer::Number(guess) => Some(guess.cmp(&self.computer.secret())),
            UserAnswer::Quite => {
                None
            }
            _ => None,
        }
    }
    ///
    pub fn run(&mut self) -> UserState {
        match self.compare() {
            Some(Ordering::Greater) => {
                println!("Слишком большое число");
                UserState::InGame
            }
            Some(Ordering::Less) => {
                println!("Слишком маленькое число");
                UserState::InGame
            }
            Some(Ordering::Equal) => {
                UserState::Win
            }
            _ => {
                UserState::Quite
            }
        }
    }
    ///
    pub fn secret(&self) -> u32 {
        self.computer.secret()
    }
}