//###########################################################################
// Copyright 2021 OceanScan - Marine Systems & Technology, Lda.             #
//###########################################################################
// Licensed under the Apache License, Version 2.0 (the "License");          #
// you may not use this file except in compliance with the License.         #
// You may obtain a copy of the License at                                  #
//                                                                          #
// http://www.apache.org/licenses/LICENSE-2.0                               #
//                                                                          #
// Unless required by applicable law or agreed to in writing, software      #
// distributed under the License is distributed on an "AS IS" BASIS,        #
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. #
// See the License for the specific language governing permissions and      #
// limitations under the License.                                           #
//###########################################################################
// Author: Tiago Sá Marques                                                 #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

use crate::Header::Header;
use crate::Message::Message;
/// Base
use crate::*;

pub fn buildFrom<T: Message>(hdr: Header) -> Option<T> {
    let mut msg: T = T::new();
    if msg.id() != hdr._mgid {
        return Option::None;
    }
    msg.get_header()._mgid = hdr._mgid;
    msg.get_header()._sync = hdr._sync;
    msg.get_header()._size = hdr._size;
    msg.get_header()._timestamp = hdr._timestamp;
    msg.get_header()._src = hdr._src;
    msg.get_header()._src_ent = hdr._src_ent;
    msg.get_header()._dst = hdr._dst;
    msg.get_header()._dst_ent = hdr._dst_ent;
    Option::Some(msg)
}

pub fn build(hdr: Header) -> Option<Box<dyn Message>> {
    let ret = buildFromId(hdr._mgid);
    if ret.is_none() {
        return ret;
    }

    let mut msg = ret.unwrap();
    msg.get_header()._dst = hdr._dst;
    msg.get_header()._size = hdr._size;
    msg.get_header()._sync = hdr._sync;
    msg.get_header()._src = hdr._src;
    msg.get_header()._dst_ent = hdr._dst_ent;
    msg.get_header()._src_ent = hdr._src_ent;
    msg.get_header()._timestamp = hdr._timestamp;

    return Option::from(msg);
}

pub fn buildFromId(id: u16) -> Option<Box<dyn Message>> {
    match id {
        1 => Option::Some(Box::new(EntityState::EntityState::new())),
        2 => Option::Some(Box::new(QueryEntityState::QueryEntityState::new())),
        3 => Option::Some(Box::new(EntityInfo::EntityInfo::new())),
        4 => Option::Some(Box::new(QueryEntityInfo::QueryEntityInfo::new())),
        5 => Option::Some(Box::new(EntityList::EntityList::new())),
        7 => Option::Some(Box::new(CpuUsage::CpuUsage::new())),
        8 => Option::Some(Box::new(TransportBindings::TransportBindings::new())),
        9 => Option::Some(Box::new(RestartSystem::RestartSystem::new())),
        10 => Option::Some(Box::new(MemUsage::MemUsage::new())),
        14 => Option::Some(Box::new(EntityActivationState::EntityActivationState::new())),
        15 => Option::Some(Box::new(
            QueryEntityActivationState::QueryEntityActivationState::new(),
        )),
        16 => Option::Some(Box::new(
            VehicleOperationalLimits::VehicleOperationalLimits::new(),
        )),
        20 => Option::Some(Box::new(MsgList::MsgList::new())),
        50 => Option::Some(Box::new(SimulatedState::SimulatedState::new())),
        100 => Option::Some(Box::new(StorageUsage::StorageUsage::new())),
        101 => Option::Some(Box::new(CacheControl::CacheControl::new())),
        102 => Option::Some(Box::new(LoggingControl::LoggingControl::new())),
        103 => Option::Some(Box::new(LogBookEntry::LogBookEntry::new())),
        104 => Option::Some(Box::new(LogBookControl::LogBookControl::new())),
        105 => Option::Some(Box::new(ReplayControl::ReplayControl::new())),
        106 => Option::Some(Box::new(ClockControl::ClockControl::new())),
        150 => Option::Some(Box::new(Heartbeat::Heartbeat::new())),
        151 => Option::Some(Box::new(Announce::Announce::new())),
        152 => Option::Some(Box::new(AnnounceService::AnnounceService::new())),
        153 => Option::Some(Box::new(RSSI::RSSI::new())),
        156 => Option::Some(Box::new(Sms::Sms::new())),
        157 => Option::Some(Box::new(SmsTx::SmsTx::new())),
        158 => Option::Some(Box::new(SmsRx::SmsRx::new())),
        159 => Option::Some(Box::new(SmsState::SmsState::new())),
        160 => Option::Some(Box::new(TextMessage::TextMessage::new())),
        170 => Option::Some(Box::new(IridiumMsgRx::IridiumMsgRx::new())),
        171 => Option::Some(Box::new(IridiumMsgTx::IridiumMsgTx::new())),
        172 => Option::Some(Box::new(IridiumTxStatus::IridiumTxStatus::new())),
        183 => Option::Some(Box::new(ExtendedRSSI::ExtendedRSSI::new())),
        200 => Option::Some(Box::new(LblRange::LblRange::new())),
        202 => Option::Some(Box::new(LblBeacon::LblBeacon::new())),
        203 => Option::Some(Box::new(LblConfig::LblConfig::new())),
        211 => Option::Some(Box::new(AcousticOperation::AcousticOperation::new())),
        212 => Option::Some(Box::new(AcousticSystemsQuery::AcousticSystemsQuery::new())),
        213 => Option::Some(Box::new(AcousticSystems::AcousticSystems::new())),
        214 => Option::Some(Box::new(AcousticLink::AcousticLink::new())),
        250 => Option::Some(Box::new(Rpm::Rpm::new())),
        251 => Option::Some(Box::new(Voltage::Voltage::new())),
        252 => Option::Some(Box::new(Current::Current::new())),
        253 => Option::Some(Box::new(GpsFix::GpsFix::new())),
        254 => Option::Some(Box::new(EulerAngles::EulerAngles::new())),
        255 => Option::Some(Box::new(EulerAnglesDelta::EulerAnglesDelta::new())),
        256 => Option::Some(Box::new(AngularVelocity::AngularVelocity::new())),
        257 => Option::Some(Box::new(Acceleration::Acceleration::new())),
        258 => Option::Some(Box::new(MagneticField::MagneticField::new())),
        259 => Option::Some(Box::new(GroundVelocity::GroundVelocity::new())),
        260 => Option::Some(Box::new(WaterVelocity::WaterVelocity::new())),
        261 => Option::Some(Box::new(VelocityDelta::VelocityDelta::new())),
        262 => Option::Some(Box::new(Distance::Distance::new())),
        263 => Option::Some(Box::new(Temperature::Temperature::new())),
        264 => Option::Some(Box::new(Pressure::Pressure::new())),
        265 => Option::Some(Box::new(Depth::Depth::new())),
        267 => Option::Some(Box::new(SoundSpeed::SoundSpeed::new())),
        268 => Option::Some(Box::new(WaterDensity::WaterDensity::new())),
        269 => Option::Some(Box::new(Conductivity::Conductivity::new())),
        270 => Option::Some(Box::new(Salinity::Salinity::new())),
        272 => Option::Some(Box::new(RelativeHumidity::RelativeHumidity::new())),
        273 => Option::Some(Box::new(DevDataText::DevDataText::new())),
        274 => Option::Some(Box::new(DevDataBinary::DevDataBinary::new())),
        276 => Option::Some(Box::new(SonarData::SonarData::new())),
        277 => Option::Some(Box::new(Pulse::Pulse::new())),
        278 => Option::Some(Box::new(PulseDetectionControl::PulseDetectionControl::new())),
        279 => Option::Some(Box::new(FuelLevel::FuelLevel::new())),
        281 => Option::Some(Box::new(ServoPosition::ServoPosition::new())),
        282 => Option::Some(Box::new(DeviceState::DeviceState::new())),
        283 => Option::Some(Box::new(BeamConfig::BeamConfig::new())),
        284 => Option::Some(Box::new(DataSanity::DataSanity::new())),
        285 => Option::Some(Box::new(RhodamineDye::RhodamineDye::new())),
        286 => Option::Some(Box::new(CrudeOil::CrudeOil::new())),
        287 => Option::Some(Box::new(FineOil::FineOil::new())),
        288 => Option::Some(Box::new(Turbidity::Turbidity::new())),
        289 => Option::Some(Box::new(Chlorophyll::Chlorophyll::new())),
        290 => Option::Some(Box::new(Fluorescein::Fluorescein::new())),
        291 => Option::Some(Box::new(Phycocyanin::Phycocyanin::new())),
        292 => Option::Some(Box::new(Phycoerythrin::Phycoerythrin::new())),
        295 => Option::Some(Box::new(DissolvedOxygen::DissolvedOxygen::new())),
        296 => Option::Some(Box::new(AirSaturation::AirSaturation::new())),
        298 => Option::Some(Box::new(PH::PH::new())),
        299 => Option::Some(Box::new(Redox::Redox::new())),
        301 => Option::Some(Box::new(SetThrusterActuation::SetThrusterActuation::new())),
        302 => Option::Some(Box::new(SetServoPosition::SetServoPosition::new())),
        304 => Option::Some(Box::new(RemoteActionsRequest::RemoteActionsRequest::new())),
        305 => Option::Some(Box::new(RemoteActions::RemoteActions::new())),
        306 => Option::Some(Box::new(ButtonEvent::ButtonEvent::new())),
        307 => Option::Some(Box::new(LcdControl::LcdControl::new())),
        308 => Option::Some(Box::new(PowerOperation::PowerOperation::new())),
        309 => Option::Some(Box::new(PowerChannelControl::PowerChannelControl::new())),
        310 => Option::Some(Box::new(
            QueryPowerChannelState::QueryPowerChannelState::new(),
        )),
        311 => Option::Some(Box::new(PowerChannelState::PowerChannelState::new())),
        312 => Option::Some(Box::new(LedBrightness::LedBrightness::new())),
        313 => Option::Some(Box::new(QueryLedBrightness::QueryLedBrightness::new())),
        314 => Option::Some(Box::new(SetLedBrightness::SetLedBrightness::new())),
        315 => Option::Some(Box::new(ChargingState::ChargingState::new())),
        350 => Option::Some(Box::new(EstimatedState::EstimatedState::new())),
        351 => Option::Some(Box::new(
            EstimatedStreamVelocity::EstimatedStreamVelocity::new(),
        )),
        354 => Option::Some(Box::new(NavigationUncertainty::NavigationUncertainty::new())),
        355 => Option::Some(Box::new(NavigationData::NavigationData::new())),
        356 => Option::Some(Box::new(GpsFixRejection::GpsFixRejection::new())),
        357 => Option::Some(Box::new(LblRangeAcceptance::LblRangeAcceptance::new())),
        358 => Option::Some(Box::new(DvlRejection::DvlRejection::new())),
        360 => Option::Some(Box::new(LblEstimate::LblEstimate::new())),
        361 => Option::Some(Box::new(AlignmentState::AlignmentState::new())),
        400 => Option::Some(Box::new(DesiredHeading::DesiredHeading::new())),
        401 => Option::Some(Box::new(DesiredZ::DesiredZ::new())),
        402 => Option::Some(Box::new(DesiredSpeed::DesiredSpeed::new())),
        403 => Option::Some(Box::new(DesiredRoll::DesiredRoll::new())),
        404 => Option::Some(Box::new(DesiredPitch::DesiredPitch::new())),
        406 => Option::Some(Box::new(DesiredPath::DesiredPath::new())),
        407 => Option::Some(Box::new(DesiredControl::DesiredControl::new())),
        408 => Option::Some(Box::new(DesiredHeadingRate::DesiredHeadingRate::new())),
        410 => Option::Some(Box::new(PathControlState::PathControlState::new())),
        411 => Option::Some(Box::new(
            AllocatedControlTorques::AllocatedControlTorques::new(),
        )),
        412 => Option::Some(Box::new(ControlParcel::ControlParcel::new())),
        413 => Option::Some(Box::new(Brake::Brake::new())),
        450 => Option::Some(Box::new(Goto::Goto::new())),
        451 => Option::Some(Box::new(PopUp::PopUp::new())),
        452 => Option::Some(Box::new(Teleoperation::Teleoperation::new())),
        453 => Option::Some(Box::new(Loiter::Loiter::new())),
        454 => Option::Some(Box::new(IdleManeuver::IdleManeuver::new())),
        456 => Option::Some(Box::new(Rows::Rows::new())),
        457 => Option::Some(Box::new(FollowPath::FollowPath::new())),
        458 => Option::Some(Box::new(PathPoint::PathPoint::new())),
        459 => Option::Some(Box::new(YoYo::YoYo::new())),
        460 => Option::Some(Box::new(TeleoperationDone::TeleoperationDone::new())),
        461 => Option::Some(Box::new(StationKeeping::StationKeeping::new())),
        462 => Option::Some(Box::new(Elevator::Elevator::new())),
        463 => Option::Some(Box::new(FollowTrajectory::FollowTrajectory::new())),
        464 => Option::Some(Box::new(TrajectoryPoint::TrajectoryPoint::new())),
        465 => Option::Some(Box::new(CustomManeuver::CustomManeuver::new())),
        468 => Option::Some(Box::new(StopManeuver::StopManeuver::new())),
        469 => Option::Some(Box::new(RegisterManeuver::RegisterManeuver::new())),
        470 => Option::Some(Box::new(ManeuverControlState::ManeuverControlState::new())),
        474 => Option::Some(Box::new(PolygonVertex::PolygonVertex::new())),
        475 => Option::Some(Box::new(CompassCalibration::CompassCalibration::new())),
        478 => Option::Some(Box::new(FollowReference::FollowReference::new())),
        479 => Option::Some(Box::new(Reference::Reference::new())),
        480 => Option::Some(Box::new(FollowRefState::FollowRefState::new())),
        483 => Option::Some(Box::new(Dislodge::Dislodge::new())),
        485 => Option::Some(Box::new(Launch::Launch::new())),
        495 => Option::Some(Box::new(Alignment::Alignment::new())),
        496 => Option::Some(Box::new(FollowCommand::FollowCommand::new())),
        497 => Option::Some(Box::new(Command::Command::new())),
        498 => Option::Some(Box::new(FollowCommandState::FollowCommandState::new())),
        499 => Option::Some(Box::new(Magnetometer::Magnetometer::new())),
        500 => Option::Some(Box::new(VehicleState::VehicleState::new())),
        501 => Option::Some(Box::new(VehicleCommand::VehicleCommand::new())),
        502 => Option::Some(Box::new(MonitorEntityState::MonitorEntityState::new())),
        503 => Option::Some(Box::new(EntityMonitoringState::EntityMonitoringState::new())),
        504 => Option::Some(Box::new(OperationalLimits::OperationalLimits::new())),
        505 => Option::Some(Box::new(GetOperationalLimits::GetOperationalLimits::new())),
        506 => Option::Some(Box::new(Calibration::Calibration::new())),
        507 => Option::Some(Box::new(ControlLoops::ControlLoops::new())),
        508 => Option::Some(Box::new(VehicleMedium::VehicleMedium::new())),
        509 => Option::Some(Box::new(Collision::Collision::new())),
        513 => Option::Some(Box::new(ReportControl::ReportControl::new())),
        515 => Option::Some(Box::new(AssetReport::AssetReport::new())),
        517 => Option::Some(Box::new(SmsRequest::SmsRequest::new())),
        518 => Option::Some(Box::new(SmsStatus::SmsStatus::new())),
        550 => Option::Some(Box::new(Abort::Abort::new())),
        551 => Option::Some(Box::new(PlanSpecification::PlanSpecification::new())),
        552 => Option::Some(Box::new(PlanManeuver::PlanManeuver::new())),
        553 => Option::Some(Box::new(PlanTransition::PlanTransition::new())),
        556 => Option::Some(Box::new(PlanDB::PlanDB::new())),
        557 => Option::Some(Box::new(PlanDBState::PlanDBState::new())),
        558 => Option::Some(Box::new(PlanDBInformation::PlanDBInformation::new())),
        559 => Option::Some(Box::new(PlanControl::PlanControl::new())),
        560 => Option::Some(Box::new(PlanControlState::PlanControlState::new())),
        561 => Option::Some(Box::new(PlanVariable::PlanVariable::new())),
        562 => Option::Some(Box::new(PlanGeneration::PlanGeneration::new())),
        564 => Option::Some(Box::new(PlanStatistics::PlanStatistics::new())),
        606 => Option::Some(Box::new(CcuEvent::CcuEvent::new())),
        650 => Option::Some(Box::new(VehicleLinks::VehicleLinks::new())),
        655 => Option::Some(Box::new(TrexOperation::TrexOperation::new())),
        656 => Option::Some(Box::new(TrexAttribute::TrexAttribute::new())),
        657 => Option::Some(Box::new(TrexToken::TrexToken::new())),
        801 => Option::Some(Box::new(EntityParameter::EntityParameter::new())),
        802 => Option::Some(Box::new(EntityParameters::EntityParameters::new())),
        803 => Option::Some(Box::new(QueryEntityParameters::QueryEntityParameters::new())),
        804 => Option::Some(Box::new(SetEntityParameters::SetEntityParameters::new())),
        805 => Option::Some(Box::new(SaveEntityParameters::SaveEntityParameters::new())),
        811 => Option::Some(Box::new(PushEntityParameters::PushEntityParameters::new())),
        812 => Option::Some(Box::new(PopEntityParameters::PopEntityParameters::new())),
        813 => Option::Some(Box::new(IoEvent::IoEvent::new())),
        814 => Option::Some(Box::new(UamTxFrame::UamTxFrame::new())),
        815 => Option::Some(Box::new(UamRxFrame::UamRxFrame::new())),
        816 => Option::Some(Box::new(UamTxStatus::UamTxStatus::new())),
        817 => Option::Some(Box::new(UamRxRange::UamRxRange::new())),
        818 => Option::Some(Box::new(UamTxRange::UamTxRange::new())),
        877 => Option::Some(Box::new(MessagePart::MessagePart::new())),
        889 => Option::Some(Box::new(Aborted::Aborted::new())),
        890 => Option::Some(Box::new(UsblAngles::UsblAngles::new())),
        891 => Option::Some(Box::new(UsblPosition::UsblPosition::new())),
        892 => Option::Some(Box::new(UsblFix::UsblFix::new())),
        898 => Option::Some(Box::new(UsblAnglesExtended::UsblAnglesExtended::new())),
        899 => Option::Some(Box::new(UsblPositionExtended::UsblPositionExtended::new())),
        900 => Option::Some(Box::new(UsblFixExtended::UsblFixExtended::new())),
        901 => Option::Some(Box::new(UsblModem::UsblModem::new())),
        902 => Option::Some(Box::new(UsblConfig::UsblConfig::new())),
        903 => Option::Some(Box::new(
            DissolvedOrganicMatter::DissolvedOrganicMatter::new(),
        )),
        904 => Option::Some(Box::new(OpticalBackscatter::OpticalBackscatter::new())),
        905 => Option::Some(Box::new(Tachograph::Tachograph::new())),
        1014 => Option::Some(Box::new(CurrentProfile::CurrentProfile::new())),
        1015 => Option::Some(Box::new(CurrentProfileCell::CurrentProfileCell::new())),
        1016 => Option::Some(Box::new(ADCPBeam::ADCPBeam::new())),
        2000 => Option::Some(Box::new(GpioState::GpioState::new())),
        2001 => Option::Some(Box::new(GpioStateGet::GpioStateGet::new())),
        2002 => Option::Some(Box::new(GpioStateSet::GpioStateSet::new())),
        2003 => Option::Some(Box::new(
            ColoredDissolvedOrganicMatter::ColoredDissolvedOrganicMatter::new(),
        )),
        2004 => Option::Some(Box::new(
            FluorescentDissolvedOrganicMatter::FluorescentDissolvedOrganicMatter::new(),
        )),
        2005 => Option::Some(Box::new(IridiumMsgTxExtended::IridiumMsgTxExtended::new())),
        2006 => Option::Some(Box::new(TotalMagIntensity::TotalMagIntensity::new())),
        2007 => Option::Some(Box::new(ValidatePlan::ValidatePlan::new())),
        2008 => Option::Some(Box::new(TypedEntityParameter::TypedEntityParameter::new())),
        2009 => Option::Some(Box::new(TypedEntityParameters::TypedEntityParameters::new())),
        2010 => Option::Some(Box::new(CommRestriction::CommRestriction::new())),
        2011 => Option::Some(Box::new(WifiStats::WifiStats::new())),
        2012 => Option::Some(Box::new(WifiNetwork::WifiNetwork::new())),
        2013 => Option::Some(Box::new(SonarPulse::SonarPulse::new())),
        2014 => Option::Some(Box::new(ValuesIf::ValuesIf::new())),
        _ => Option::None,
    }
}
