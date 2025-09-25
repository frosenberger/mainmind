use crate::colors::{Color, Match};

#[derive(Debug, Default)]
pub struct Game {
    code: Vec<Color>,
    pub guesses: Vec<Vec<Color>>,
    pub matches: Vec<Vec<Match>>,
    max_rounds: u8,
    pub round: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            code: gen_code(4),
            guesses: Vec::new(),
            matches: Vec::new(),
            max_rounds: 7,
            round: 1,
        }
    }

    pub fn check_guess(&mut self, guess: &Vec<Color>) -> Result<(), Vec<Match>> {
        self.guesses.push(guess.to_vec());
        let mut matches: Vec<Match> = Vec::new();
        for i in 0..guess.len() {
            let guess_at_i = guess.get(i).unwrap();
            let contained = self.code.contains(guess_at_i);
            let contained_at_i = self.code.get(i).unwrap() == guess_at_i;
            if contained_at_i {
                matches.push(Match::FULL);
            } else if contained {
                matches.push(Match::PARTIAL);
            } else {
                matches.push(Match::NO);
            }
        }
        self.matches.push(matches.clone());
        if matches.iter().all(|m| *m == Match::FULL) {
            Ok(())
        } else {
            self.round += 1;
            Err(matches)
        }
    }
}

pub fn gen_code(length: usize) -> Vec<Color> {
    let mut code = Vec::with_capacity(length);
    for _i in 0..length {
        code.push(Color::random());
    }
    code
}

mod test {
    use super::*;

    #[test]
    fn test_game() {
        let mut game = Game::new();
        let code = game.code.clone();
        let mut diff_code = gen_code(4);
        while diff_code == code {
            diff_code = gen_code(4)
        }
        assert_eq!(game.check_guess(&code), Ok(()));
        assert_ne!(game.check_guess(&diff_code), Ok(()));
    }
}
