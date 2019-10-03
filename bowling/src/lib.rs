#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    bowls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { bowls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.game_complete() {
            Err(Error::GameComplete)
        } else if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else {
            match self.bowls.len() {
                0..=17 if self.bowls.len() % 2 == 0 => {
                    self.bowls.push(pins);
                    if pins == 10 {
                        self.bowls.push(0);
                    }
                    Ok(())
                },
                0..=17 if self.bowls.len() % 2 != 0 => {
                    if pins + self.bowls.last().unwrap() > 10 {
                        Err(Error::NotEnoughPinsLeft)
                    } else {
                        self.bowls.push(pins);
                        Ok(())
                    }
                },
                18 => {
                    self.bowls.push(pins);
                    Ok(())
                },
                19 => if self.bowls[18] != 10 && self.bowls[18] + pins > 10 {
                    Err(Error::NotEnoughPinsLeft)
                } else {
                    self.bowls.push(pins);
                    Ok(())
                },
                20 => if self.bowls[18] == 10 && self.bowls[19] != 10 && self.bowls[19] + pins > 10 {
                    Err(Error::NotEnoughPinsLeft)
                } else {
                    self.bowls.push(pins);
                    Ok(())
                },
                _ => unreachable!(), 
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.game_complete() {
            let mut score = 0;
            for (i, roll) in self.bowls.iter().enumerate().filter(|&(index, _)| index < 18) {
                score += roll;
                if *roll == 10 {            // when theres a strike
                    let mut remaining = 2;
                    for (_, val) in self.bowls.iter().enumerate().filter(|&(index, val)| index > i && *val != 0) {
                        score += val;
                        remaining -= 1;
                        if remaining == 0 {
                            break;
                        }
                    }
                }
                if i % 2 != 0 {     // check for spare
                    if roll + self.bowls[i - 1] == 10 && *roll != 10 && self.bowls[i - 1] != 10 {   // spare found
                        if let Some(next_val) = self.bowls
                            .iter()
                            .enumerate()
                            .filter(|&(index, val)| index > i && *val != 0)
                            .map(|(_, val)| *val)
                            .next() {
                                score += next_val;    
                        }
                    }
                }
            }
            for (_, roll) in self.bowls.iter().enumerate().filter(|&(index, _)| index >= 18) {
                score += roll;
            }
            Some(score)
        } else {
            None
        }
    }

    pub fn game_complete(&self) -> bool {
        match self.bowls.len() {
            0..=19 => false,
            20 => self.bowls[18] + self.bowls[19] < 10,
            _ => true,
        }
    }
}
