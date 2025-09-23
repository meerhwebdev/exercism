use core::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours:i32,
    minutes:i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut calculated_minutes = ((hours * 60) +  minutes) % 60;
        let mut _minutes_to_add  = 0;
        if calculated_minutes < 0 {
            _minutes_to_add = calculated_minutes + 60;
            calculated_minutes += 60;
        } else {
            _minutes_to_add = calculated_minutes
        }
        let hours_to_add = (minutes - _minutes_to_add) / 60;
        let mut calculated_hours = (hours + hours_to_add) % 24;
        if calculated_hours < 0 {
            calculated_hours += 24
        }
        Self {
            hours:calculated_hours,
            minutes:calculated_minutes
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        let mut calculated_minutes = ((self.hours * 60) +  self.minutes) % 60;
        let mut _minutes_to_add  = 0;
        if calculated_minutes < 0 {
            _minutes_to_add = calculated_minutes + 60;
            calculated_minutes += 60;
        } else {
            _minutes_to_add = calculated_minutes
        }
        let hours_to_add = (self.minutes - _minutes_to_add) / 60;
        let mut calculated_hours = (self.hours + hours_to_add) % 24;
        if calculated_hours < 0 {
            calculated_hours += 24
        }
        Self {
            hours:calculated_hours,
            minutes:calculated_minutes
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f:&mut fmt::Formatter<'_>)-> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours,self.minutes)
    }
}