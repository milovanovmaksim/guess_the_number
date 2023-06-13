use std::io;

pub enum UserState {
    InGame,
    Win,
    GameOver,
    Quite,
}

pub enum UserAnswer {
    Quite,
    Number(u32),
    Yes,
    No,
}

pub struct User {}


impl User {
    pub fn new() -> Self {
        Self {}
    }
    pub fn guess(&self) -> UserAnswer {
        loop {
            println!("\nПожалуйста, введите свою догадку. Для выхода из игры нажмите \"q\"");
            let mut guess = String::new();
            match io::stdin().read_line(&mut guess) {
                Ok(_) => {
                    if guess.trim() == "q" {
                        return UserAnswer::Quite;
                    }
                    match guess.trim().parse() {
                        Ok(guess) => return UserAnswer::Number(guess),
                        Err(_) => {
                            println!("Пожалуйста, наберите число!");
                        }
                    }
                }
                Err(_) => {
                    println!("Не получилось прочитать строку.");
                }
            }
        }
    }
}
