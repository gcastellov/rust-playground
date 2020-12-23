const ALL_PINS: u16 = 10;
const NUM_OF_FRAMES: usize = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Copy, Clone)]
struct Frame {
    rolls: [Option<u16>; 2]
}

pub struct BowlingGame {
    completed_frames: Vec<Frame>,
    current_frame: Frame,
    previous_frame: Option<Frame>
}

impl Frame {
    fn new() -> Self {
        Frame { rolls: [None; 2] }
    }

    fn is_strike(&self) -> bool {
        self.sum_of_frame() == ALL_PINS && self.rolls.iter().filter(|r|r.is_some()).count() == 1
    }

    fn is_spare(&self) -> bool {
        self.sum_of_frame() == ALL_PINS && self.rolls.iter().all(|r|r.is_some())
    }

    fn is_completed(&self) -> bool {
        self.is_strike() || self.rolls.iter().all(|r|r.is_some())
    }

    fn sum_of_frame(&self) -> u16 {
        self.rolls.iter().map(|roll|roll.unwrap_or(0)).sum()
    }

    fn next(&mut self, pins: u16) -> Result<u16, Error> {
        if self.sum_of_frame() + pins > ALL_PINS {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.rolls[0].is_none() {
            self.rolls[0] = Some(pins);
        } else {
            self.rolls[1] = Some(pins);
        }

        Ok(self.sum_of_frame())
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            completed_frames: Vec::default(),
            current_frame: Frame::new(),
            previous_frame: None
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {

        self.current_frame.next(pins)?;
        let count_completed_frames = self.completed_frames.len();

        if count_completed_frames >= 10 {
            let prev_frame = self.previous_frame.unwrap();
            if !prev_frame.is_spare() && !prev_frame.is_strike() {
                return Err(Error::GameComplete)
            }
        }

        if self.current_frame.is_completed() || (count_completed_frames == 10 && !self.previous_frame.unwrap().is_strike()){
            self.completed_frames.push(self.current_frame);
            self.previous_frame = Some(self.current_frame);
            self.current_frame = Frame::new();
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        match self.completed_frames.len() {
            12 => Some(self.calc_score()),
            11 => 
                if self.completed_frames[9].is_strike() && self.completed_frames[10].is_strike() {
                    None
                } else {
                    Some(self.calc_score())
                }
            ,
            10 =>                 
                if self.completed_frames[9].sum_of_frame() == ALL_PINS {
                    None
                } else {
                    Some(self.calc_score())
                }
            ,
            _ => None
        }
    }

    fn calc_score(&self) -> u16 {
        let mut score = 0;

        for frame_index in 0..NUM_OF_FRAMES {
            let frame = self.completed_frames[frame_index];
            score += frame.sum_of_frame();
            if frame.is_strike() {
                score += self.strike_bonus(frame_index, &self.completed_frames);
            } else if frame.is_spare() {
                score += self.spare_bonus(&self.completed_frames[frame_index+1]);
            }
        }

        score
    }

    fn spare_bonus(&self, next_frame: &Frame) -> u16 {
        next_frame.rolls[0].unwrap()
    }

    fn strike_bonus(&self, frame_index: usize, next_frames: &[Frame]) -> u16 {
        let mut score = 0;
        if let Some(frame) = next_frames.get(frame_index+1) {
            score += frame.sum_of_frame();
            if frame.is_strike() {
                if let Some(frame_two) = next_frames.get(frame_index+2) {
                    if frame_two.is_strike() || frame_two.is_spare() {
                        score += frame_two.sum_of_frame();
                    } else {
                        score += frame_two.rolls[0].unwrap();
                    }
                }
            }
        }
        
        score
    }
}
