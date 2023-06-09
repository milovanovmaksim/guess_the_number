use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::io;
use std::rc::Rc;

enum UserState {
    InGame,
    Win,
    GameOver,
    Quite,
}

enum UserAnswer {
    Quite,
    Number(u32),
    Yes,
    No,
}

struct User {
    state: UserState,
}

impl User {
    fn guess(&self) -> UserAnswer {
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

    fn change_state(&mut self, state: UserState) {
        self.state = state;
    }
}

struct Computer {
    secret: u32,
}

impl Computer {
    fn think_the_secret(&mut self) {
        self.secret = rand::thread_rng().gen_range(1..100);
    }
}

struct Engine {
    user:  Rc<RefCell<User>>,
    computer: Computer,
}

impl Engine {
    fn compare(&self) -> Option<Ordering> {
        match self.user.borrow().guess() {
            UserAnswer::Number(guess) => Some(guess.cmp(&self.computer.secret)),
            UserAnswer::Quite => {
                self.user.borrow_mut().change_state(UserState::Quite);
                None
            }
            _ => None,
        }
    }
}

struct Game {
    attempt: Attempt,
    user: Rc<RefCell<User>>,
}

impl Game {
    fn run(&mut self) {
        println!("Отгадайте число от 0 до 100, которое загодал компьютер.");
        while self.attempt.attempts > 0 {
            self.attempt.attempt();
            match self.user.borrow().state {
                UserState::GameOver => {
                    println!("Вы проиграли. Загаданное число - {}", self.secret());
                }
                UserState::Win => {
                    println!("Вы выиграли!");
                }
                UserState::Quite => {
                    break;
                }
                _ => { continue; }
            }
            match self.ask_user_for_restarting_game() {
                UserAnswer::Yes => {
                    self.restart();
                }
                _ => { }
            }
        }
        println!("Вы покинули игру.");
    }

    fn restart(&mut self) {
        self.attempt.attempts = 6;
        self.attempt.verbose_diff.engine.computer.think_the_secret();
        self.user.borrow_mut()
            .change_state(UserState::InGame);
        println!("Отгадайте число от 0 до 100, которое загодал компьютер.");
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

    fn secret(&self) -> u32 {
        self.attempt.verbose_diff.engine.computer.secret
    }
}

struct Attempt {
    attempts: u32,
    verbose_diff: VerboseDiff,
}

impl Attempt {
    fn attempt(&mut self) {
        self.attempts -= 1;
        self.verbose_diff.print_info(self.attempts);
    }
}

struct VerboseDiff {
    engine: Engine,
}

impl VerboseDiff {
    fn print_info(&self, attempts: u32) {
        match self.engine.compare() {
            Some(Ordering::Greater) => {
                println!("Слишком большое число");
                if attempts == 0 {
                    self.engine.user.borrow_mut().change_state(UserState::GameOver);
                }
                println!("Количество оставшихся попыток - {}", attempts);
            }
            Some(Ordering::Less) => {
                println!("Слишком маленькое число");
                if attempts == 0 {
                    self.engine.user.borrow_mut().change_state(UserState::GameOver);
                }
                println!("Количество оставшихся попыток - {}", attempts);
            }
            Some(Ordering::Equal) => {
                self.engine.user.borrow_mut().change_state(UserState::Win);
                println!("Количество оставшихся попыток - {}", attempts);
            }
            _ => {}
        }
    }
}

fn main() {
    let secret = rand::thread_rng().gen_range(0..100);
    // println!("{}", secret);
    let computer = Computer { secret };
    let user = Rc::new(RefCell::new(User {
        state: UserState::InGame,
    }));
    let engine = Engine { user: user.clone(), computer };
    let mut game = Game {
        attempt: Attempt {
            attempts: 6,
            verbose_diff: VerboseDiff { engine },
        }, user: user.clone()
    };
    game.run();
}


// user = User { };
// Game {
//     Attempt{
//         VerboseDiff {
//             Engine {
//                 user,
//                 Computer{}
//             }
//         }, 6
//     }, user
// }.run();
