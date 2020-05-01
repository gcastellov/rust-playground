use std::fmt;

const HOURS: i32 = 24;
const MINUTES: i32 = 60;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let mut _hours = match minutes.checked_div_euclid(MINUTES) {
            Some(h) => hours + h,
            _ => hours
        };

        let _minutes = match minutes.checked_rem_euclid(MINUTES) {
            Some(m) => m,
            _ => minutes
        };

        _hours = match _hours.checked_rem_euclid(HOURS) {
            Some(h) => h,
            _ => 0
        };
        
        Clock { 
            hours: _hours, 
            minutes: _minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let add_minutes: i32 = self.minutes + minutes;
        Self::new(self.hours, add_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}