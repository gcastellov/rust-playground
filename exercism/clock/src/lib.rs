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

        let mut _hours = hours;
        let mut _minutes = minutes;
        let mm = _minutes % MINUTES;
        _hours += _minutes / MINUTES;
        
        if mm < 0 {
            _hours -=1;
        }

        let hh = _hours % HOURS;
        let get_hours = |hh| (if hh >= 0 { hh } else if hh < 0 {  HOURS + hh } else { _hours });
        let get_minutes = |mm| (if mm >= 0 { mm } else if mm < 0 { MINUTES + mm } else { _minutes } );

        Clock { 
            hours: get_hours(hh), 
            minutes: get_minutes(mm)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let add_minutes: i32 = self.minutes + minutes;
        Self::new(self.hours, add_minutes)
    }

    fn show(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.show(f)
    }
}