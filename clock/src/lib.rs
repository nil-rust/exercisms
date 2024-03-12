use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Self { hours: 0, minutes: 0 };

        clock.add_hours(hours);
        clock.add_minutes(minutes);

        clock
    }

    fn add_hours(&mut self, hours: i32) {
        let hour = hours.rem_euclid(24);

        let new_hour = self.hours + hour;

        if new_hour >= 24 {
            self.hours = new_hour.rem_euclid(24);
        } else if new_hour < 0 {
            self.hours = 24 - new_hour
        } else {
            self.hours = new_hour
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Clock {
        let hour = minutes / 60;
        let mut minutes_to_add = minutes.abs().rem_euclid(60);
        if minutes < 0 {
            minutes_to_add *= -1
        }

        self.add_hours(hour);

        self.minutes += minutes_to_add;

        if self.minutes >= 60 {
            self.add_hours(1);
            self.minutes = self.minutes - 60
        } else if self.minutes < 0 {
            self.add_hours(-1);
            self.minutes = 60 + self.minutes;
        }

        Self {
            hours: self.hours,
            minutes: self.minutes,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
