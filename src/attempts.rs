use crate::{verbose_diff::VerboseDiff, user::UserState};

pub struct Attempts {
    attempts: u32,
    verboseDiff: VerboseDiff,
}

impl Attempts {
    ///
    pub fn new(
        attempts: u32,
        verboseDiff: VerboseDiff,
    ) -> Self {
        Self {
            attempts,
            verboseDiff,
        }
    }
    ///
    pub fn run(&mut self) -> UserState {
        let mut result = UserState::GameOver;
        self.verboseDiff.restart();
        let mut attemptsElapsed = self.attempts;
        while attemptsElapsed > 0 {
            println!("\tосталось попыток: {}", attemptsElapsed);
            match self.verboseDiff.run() {
                UserState::InGame => {
                    attemptsElapsed -= 1;
                },
                UserState::GameOver => {
                    result = UserState::GameOver;
                    attemptsElapsed = 0;
                },
                UserState::Quite => {
                    result = UserState::Quite;
                    attemptsElapsed = 0;

                },
                UserState::Win => {
                    result = UserState::Win;
                    attemptsElapsed = 0;
                },
            };
        }
        result
    }
    ///
    pub fn secret(&self) -> u32 {
        self.verboseDiff.secret()
    }
}

