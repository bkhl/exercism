use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let minutes = hours * 60 + minutes;
        let clock = Clock {
            hours: 0,
            minutes: 0,
        };
        clock.add_minutes(minutes)
    }

    pub fn add_minutes(mut self, minutes: i32) -> Clock {
        self.minutes += minutes;

        while self.minutes >= 60 {
            self.minutes -= 60;
            self.hours += 1;
        }

        while self.minutes <= -60 {
            self.minutes += 60;
            self.hours -= 1;
        }

        while self.minutes < 0 {
            self.minutes += 60;
            self.hours -= 1;
        }

        while self.hours < 0 {
            self.hours += 24;
        }

        while self.hours >= 24 {
            self.hours -= 24;
        }

        self
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}