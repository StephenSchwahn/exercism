#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Copy, Clone, Debug)]
pub struct Frame {
    first: Option<u16>,
    second: Option<u16>,
    third: Option<u16>,
}

pub struct BowlingGame {
    current_frame: u16,
    frames: [Frame; 10],
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_frame: 0,
            frames: [Frame {
                first: Option::None,
                second: Option::None,
                third: Option::None,
            }; 10],
        }
    }

    fn frame_total(&self, idx: usize) -> u16 {
        self.frames[idx].first.unwrap_or_default()
            + self.frames[idx].second.unwrap_or_default()
            + self.frames[idx].third.unwrap_or_default()
    }

    fn is_game_complete(&self) -> bool {
        self.frames.iter().all(|frame| frame.first.is_some()) // If every frame has one value
            && self
                .frames
                .last()
                .map(|frame| {
                    if (frame.first == Some(10) && frame.second == Some(10) && frame.third.is_some()) || 
                       (frame.first == Some(10) && frame.second.is_some() && frame.third.is_some()) ||
                       (frame.first.unwrap_or_default() + frame.second.unwrap_or_default() == 10 && frame.third.is_some()) ||
                       (frame.first.is_some() && frame.first != Some(10) && frame.second.is_some() && frame.first.unwrap_or_default() + frame.second.unwrap_or_default() != 10) 
                    {
                        true
                    } else {
                        false
                    }
                })
                .unwrap()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.current_frame == 10 || self.is_game_complete() {
            Err(Error::GameComplete)
        } else if self.current_frame != 9
            && self.frame_total(self.current_frame as usize) + pins > 10
        {
            Err(Error::NotEnoughPinsLeft)
        } else {
            let frame = &mut self.frames[self.current_frame as usize];
            if frame.first.is_none() && pins <= 10 {
                frame.first = Some(pins);
            } else if frame.second.is_none() && pins <= 10 {
                frame.second = Some(pins);
            } else if (frame.first == Some(10) && frame.second == Some(10) && pins <= 10)
                || (frame.first == Some(10) && frame.second.unwrap_or_default() + pins <= 10)
                || (frame.first.unwrap_or_default() + frame.second.unwrap_or_default() == 10
                    && pins <= 10)
            {
                frame.third = Some(pins);
                self.current_frame += 1
            } else {
                return Err(Error::NotEnoughPinsLeft);
            }

            if self.current_frame < 9
                && ((frame.first.is_some() && frame.first == Some(10))
                    || (frame.first.is_some() && frame.second.is_some()))
            {
                self.current_frame += 1
            }
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_complete() {
            let mut total = 0;
            for (idx, frame) in self.frames.iter().enumerate() {
                if idx == 9 {
                    // Last frame is a simple sum
                    total += frame.first.unwrap_or_default()
                        + frame.second.unwrap_or_default()
                        + frame.third.unwrap_or_default()
                } else {
                    match (frame.first, frame.second) {
                        (Some(10), None) => {
                            // Strike has several edge cases.
                            // Seek for the next non-none optional value after next.first
                            // On the 9th frame, this means protecting against overpeeking
                            total += 10
                                + self.frames[idx + 1].first.unwrap_or_default()
                                + self.frames[idx + 1]
                                    .second
                                    .or_else(|| self.frames[idx + 2].first) // use or_else to prevent eagerly loading past end of array
                                    .unwrap_or_default();
                        }
                        (Some(x), Some(y)) if x + y == 10 => {
                            // Spare
                            total += 10 + self.frames[idx + 1].first.unwrap_or_default()
                        }
                        (Some(x), Some(y)) => total += x + y,
                        _ => { /* Noop */ }
                    }
                }
            }
            Some(total)
        } else {
            None
        }
    }
}
