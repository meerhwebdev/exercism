use std::{cell::RefCell};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
struct Frame {
    rolls:Vec<u16>,
    is_strike:bool,
    is_spare:bool,
    is_tenth:bool,
    is_completed:bool
}

impl Frame {
    fn new(is_tenth:bool)-> Self {
        Self {
            rolls:vec![],
            is_spare:false,
            is_strike:false,
            is_completed:false,
            is_tenth:is_tenth
        }
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    frames:Vec<RefCell<Frame>>
}

#[allow(dead_code)]
impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames:vec![RefCell::new(Frame::new(false))]
        }
    }

    fn add_frame(&mut self, length:usize) {
        self.frames.push(RefCell::new(Frame::new(length == 9)));
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {return Err(Error::NotEnoughPinsLeft)}
        let frames_len = self.frames.len();
        let is_completed = self.frames.last().unwrap().borrow().is_completed;
        if frames_len < 10 && is_completed {
            self.add_frame(frames_len);
        }
        if frames_len == 10 && is_completed {
            return Err(Error::GameComplete)
        }
        let frame=  self.frames.last().unwrap();
        let mut last_frame = frame.borrow_mut();
        if !last_frame.is_completed {
            if !last_frame.is_tenth {
                if pins == 10 {
                    last_frame.rolls.push(pins);
                    last_frame.is_completed = true;
                    last_frame.is_strike= true;
                } else {
                    if last_frame.rolls.is_empty() {
                        last_frame.rolls.push(pins);
                    }else {
                        last_frame.rolls.push(pins);
                        last_frame.is_completed = true;
                        let acc_score = last_frame.rolls.iter().fold(0, |acc, score| acc + score);
                        if acc_score > 10  {return Err(Error::NotEnoughPinsLeft)}
                        last_frame.is_spare = acc_score == 10;
                    }
                }
            }else {
                    let last_frame_rolls_len = last_frame.rolls.len();
                    if last_frame.is_strike {
                        if pins == 10 && last_frame.rolls[last_frame_rolls_len-1] != 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        if last_frame_rolls_len == 2 && last_frame.rolls[1] != 10 && pins + last_frame.rolls[1] > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        last_frame.rolls.push(pins);
                        last_frame.is_completed = if last_frame_rolls_len == 2 {true} else {false}
                    } else if last_frame.is_spare {
                        last_frame.rolls.push(pins);
                        last_frame.is_completed = if last_frame_rolls_len == 2 {true} else {false}
                    }else {
                        last_frame.rolls.push(pins);
                        last_frame.is_strike= if pins == 10 {true} else {false};
                        let acc_score = last_frame.rolls.iter().fold(0, |acc, score| acc + score);
                        if acc_score > 10  {return Err(Error::NotEnoughPinsLeft)}
                        last_frame.is_spare = !last_frame.is_strike && acc_score == 10;
                        last_frame.is_completed =  !last_frame.is_spare && !last_frame.is_strike && last_frame_rolls_len == 1
                    }
            }
        }
        Ok(())
    }

    fn next_rolls(&self, scores:&mut Vec<u16>, next:usize, frame_index:usize){
        let rolls = &self.frames[frame_index].borrow().rolls;
        let rolls_len = rolls.len();
        if rolls_len >= next {
            scores.extend(&rolls[..next]);
        }else {
            scores.extend(&rolls[..next-1]);
            self.next_rolls(scores, next - 1, frame_index+1);
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut scores:Vec<u16> = Vec::new();
        let frames = &self.frames;
        let last_frame = frames.last().unwrap().borrow();
        if !last_frame.is_tenth || !last_frame.is_completed {
            None
        }else {
           for i in 0..10 {
            let frame = &frames[i].borrow();
            if !frame.is_tenth && (frame.is_spare || frame.is_strike) {
                scores.extend(&frame.rolls);
                self.next_rolls(&mut scores, if frame.is_spare {1} else {2}, i+1);
            }else {
                scores.extend(&frame.rolls);
            }
           }
           Some(scores.iter().fold(0, |acc, score| acc + score))
        }
    }
}