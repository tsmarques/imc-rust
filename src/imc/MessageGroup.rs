use crate::imc::Message::Message;

pub trait ControlCommand: Message {}

pub trait Maneuver: Message {}
