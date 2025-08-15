use chrono::Duration;
use colored::*;
use std::fmt::{Result as Res, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self,  format:&mut Formatter) ->Res {
        let color  = self.content.truecolor(self.color.0, self.color.1, self.color.2);
        write!(format,"({:?}, {}, {})", self.position, self.size, color)
    }
}

use Event::*;

impl Event<'_> {
	pub fn notify(&self) -> Notification {
        match self {
            Remainder(text)=> Notification{
                size :50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
                },
            Registration(duration) => {
                let converted = duration.num_seconds();
                let minutes=(converted%3600)/60;
                let seconds = converted%60;
                let hours= converted /3600;
                let formatted = format!("{}H:{}M:{}S", hours, minutes, seconds);
                    Notification{
                    size :30,
                    color: (255, 2, 22 ),
                    position: Position::Top,
                    content: format!("You have {} left before the registration ends", formatted),
                }
            }, 
            Appointment(text)=> Notification{
                size :100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Holiday => Notification{
                size :25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            }
        }
	}
}