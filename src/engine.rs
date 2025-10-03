use crate::code::{Code, Match};

#[derive(Debug)]
pub struct Game {
    code: Vec<Code>,
    pub code_length: u8,
    pub guesses: Vec<Vec<Code>>,
    pub matches: Vec<Vec<Match>>,
    pub max_rounds: u8,
    pub round: u8,
    pub won: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            code: gen_code(4),
            code_length: 4,
            guesses: Vec::new(),
            matches: Vec::new(),
            max_rounds: 7,
            round: 1,
            won: false,
        }
    }

    pub fn add_guess(&mut self, guess: &Vec<Code>) {
        self.guesses.push(guess.to_vec());
        let result = self.check_guess(guess);
        match result {
            Ok(_) => self.won = true,
            Err(matches) => self.matches.push(matches),
        }
        self.round += 1;
    }

    pub fn check_guess(&mut self, guess: &Vec<Code>) -> Result<(), Vec<Match>> {
        let mut matches: Vec<Match> = Vec::new();
        for i in 0..guess.len() {
            let guess_at_i = guess.get(i).unwrap();
            let contained = self.code.contains(guess_at_i);
            let contained_at_i = self.code.get(i).unwrap() == guess_at_i;
            if contained_at_i {
                matches.push(Match::Full);
            } else if contained {
                matches.push(Match::Partial);
            } else {
                matches.push(Match::No);
            }
        }
        if matches.iter().all(|m| *m == Match::Full) {
            Ok(())
        } else {
            Err(matches)
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

pub fn gen_code(length: usize) -> Vec<Code> {
    let mut code = Vec::with_capacity(length);
    for _i in 0..length {
        code.push(Code::random());
    }
    code
}

mod test {
    #[test]
    fn test_game() {
        let mut game = super::Game::new();
        let code = game.code.clone();
        let mut diff_code = super::gen_code(4);
        while diff_code == code {
            diff_code = super::gen_code(4)
        }
        assert_eq!(game.check_guess(&code), Ok(()));
        assert_ne!(game.check_guess(&diff_code), Ok(()));
    }
}
