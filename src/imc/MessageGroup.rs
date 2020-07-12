use crate::imc::Message::Message;

pub trait Maneuver: Message {}

pub trait ControlCommand: Message {}
