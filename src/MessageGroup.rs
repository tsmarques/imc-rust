use crate::Message::Message;

pub trait Maneuver: Message {}

pub trait ControlCommand: Message {}
