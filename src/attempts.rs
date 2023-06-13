use crate::{verbose_diff::VerboseDiff, user::UserState};

pub struct Attempts {
    count: u32,
    verboseDiff: VerboseDiff,
}

impl Attempts {
    ///
    pub fn new(
        count: u32,
        verboseDiff: VerboseDiff,
    ) -> Self {
        Self {
            count,
            verboseDiff,
        }
    }
    ///
    pub fn run(&mut self) -> UserState {
        let mut result = UserState::GameOver;
        self.verboseDiff.restart();
        let mut attemptsRemained = self.count;
        while attemptsRemained > 0 {
            println!("\tосталось попыток: {}", attemptsRemained);
            match self.verboseDiff.run() {
                UserState::InGame => {
                    attemptsRemained -= 1;
                },
                UserState::GameOver => {
                    result = UserState::GameOver;
                    attemptsRemained = 0;
                },
                UserState::Quite => {
                    result = UserState::Quite;
                    attemptsRemained = 0;

                },
                UserState::Win => {
                    result = UserState::Win;
                    attemptsRemained = 0;
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

