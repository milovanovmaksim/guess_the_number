#![allow(non_snake_case)]
mod user;
mod game;
mod computer;
mod attempts;
mod verbose_diff;

use attempts::Attempts;
use computer::Computer;
use user::User;
use verbose_diff::VerboseDiff;
use game::Game;


fn main() {
    Game::new(
        Attempts::new(
            6,
            VerboseDiff::new(
                User {}, 
                Computer::new(0..100),
            ),
        ), 
    ).run();
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
