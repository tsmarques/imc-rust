use crate::Message::Message;
use crate::{Abort, Alignment, CompassCalibration, CustomManeuver, DesiredHeading, DesiredPath, DesiredPitch, DesiredRoll, DesiredSpeed, DesiredZ, Dislodge, Elevator, FollowCommand, FollowPath, FollowReference, FollowTrajectory, Goto, IdleManeuver, Launch, Loiter, Magnetometer, PopUp, Rows, StationKeeping, Teleoperation, YoYo};

#[derive(Clone)]
pub enum ControlCommand {
    ImcDesiredZ(DesiredZ::DesiredZ),
    ImcDesiredHeading(DesiredHeading::DesiredHeading),
    ImcDesiredRoll(DesiredRoll::DesiredRoll),
    ImcDesiredPitch(DesiredPitch::DesiredPitch),
    ImcDesiredSpeed(DesiredSpeed::DesiredSpeed),
    ImcDesiredPath(DesiredPath::DesiredPath),
}

impl ControlCommand {
    pub fn get_inner(&self) -> &dyn Message {
        match self {
            ControlCommand::ImcDesiredZ(m) => m,
            ControlCommand::ImcDesiredHeading(m) => m,
            ControlCommand::ImcDesiredRoll(m) => m,
            ControlCommand::ImcDesiredPitch(m) => m,
            ControlCommand::ImcDesiredSpeed(m) => m,
            ControlCommand::ImcDesiredPath(m) => m,
        }
    }
}

#[derive(Clone)]
pub enum Maneuver {
    ImcAlignment(Alignment::Alignment),
    ImcCustomManeuver(CustomManeuver::CustomManeuver),
    ImcCompassCalibration(CompassCalibration::CompassCalibration),
    ImcDislodge(Dislodge::Dislodge),
    ImcElevator(Elevator::Elevator),
    ImcFollowCommand(FollowCommand::FollowCommand),
    ImcFollowPath(FollowPath::FollowPath),
    ImcFollowReference(FollowReference::FollowReference),
    ImcFollowTrajectory(FollowTrajectory::FollowTrajectory),
    ImcGoto(Goto::Goto),
    ImcIdleManeuver(IdleManeuver::IdleManeuver),
    ImcLaunch(Launch::Launch),
    ImcLoiter(Loiter::Loiter),
    ImcMagnetometer(Magnetometer::Magnetometer),
    ImcPopUp(PopUp::PopUp),
    ImcTeleoperation(Teleoperation::Teleoperation),
    ImcRows(Rows::Rows),
    ImcStationKeeping(StationKeeping::StationKeeping),
    ImcYoYo(YoYo::YoYo),
}

impl Maneuver {
    pub fn get_inner(&self) -> &dyn Message {
        match self {
            Maneuver::ImcAlignment(m) => m,
            Maneuver::ImcCustomManeuver(m) => m,
            Maneuver::ImcCompassCalibration(m) => m,
            Maneuver::ImcDislodge(m) => m,
            Maneuver::ImcElevator(m) => m,
            Maneuver::ImcFollowCommand(m) => m,
            Maneuver::ImcFollowPath(m) => m,
            Maneuver::ImcFollowReference(m) => m,
            Maneuver::ImcFollowTrajectory(m) => m,
            Maneuver::ImcGoto(m) => m,
            Maneuver::ImcIdleManeuver(m) => m,
            Maneuver::ImcLaunch(m) => m,
            Maneuver::ImcLoiter(m) => m,
            Maneuver::ImcMagnetometer(m) => m,
            Maneuver::ImcPopUp(m) => m,
            Maneuver::ImcTeleoperation(m) => m,
            Maneuver::ImcRows(m) => m,
            Maneuver::ImcStationKeeping(m) => m,
            Maneuver::ImcYoYo(m) => m,
        }
    }
}
