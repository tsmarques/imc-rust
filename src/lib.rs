#![allow(non_snake_case)]
#![allow(dead_code)]

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
// IMC XML MD5: 3ec4b61a1b487d356bfc62e124f22651                            *
//###########################################################################

/// Base
pub mod Header;
pub mod Message;
pub mod MessageGroup;
pub mod factory;
pub mod packet;

pub type MessageList<T> = Vec<T>;

pub mod Abort;
pub mod Aborted;
pub mod Acceleration;
pub mod AcousticLink;
pub mod AcousticOperation;
pub mod AcousticSystems;
pub mod AcousticSystemsQuery;
pub mod AirSaturation;
pub mod Alignment;
pub mod AlignmentState;
pub mod AllocatedControlTorques;
pub mod AngularVelocity;
pub mod Announce;
pub mod AnnounceService;
pub mod BeamConfig;
pub mod Brake;
pub mod ButtonEvent;
pub mod CacheControl;
pub mod Calibration;
pub mod CcuEvent;
pub mod Chlorophyll;
pub mod ClockControl;
pub mod Collision;
pub mod ColoredDissolvedOrganicMatter;
pub mod Command;
pub mod CompassCalibration;
pub mod Conductivity;
pub mod ControlLoops;
pub mod ControlParcel;
pub mod CpuUsage;
pub mod CrudeOil;
pub mod Current;
pub mod CustomManeuver;
pub mod DataSanity;
pub mod Depth;
pub mod DesiredControl;
pub mod DesiredHeading;
pub mod DesiredHeadingRate;
pub mod DesiredPath;
pub mod DesiredPitch;
pub mod DesiredRoll;
pub mod DesiredSpeed;
pub mod DesiredZ;
pub mod DevDataBinary;
pub mod DevDataText;
pub mod DeviceState;
pub mod Dislodge;
pub mod DissolvedOrganicMatter;
pub mod DissolvedOxygen;
pub mod Distance;
pub mod DvlRejection;
pub mod Elevator;
pub mod EntityActivationState;
pub mod EntityInfo;
pub mod EntityList;
pub mod EntityMonitoringState;
pub mod EntityParameter;
pub mod EntityParameters;
pub mod EntityState;
pub mod EstimatedState;
pub mod EstimatedStreamVelocity;
pub mod EulerAngles;
pub mod EulerAnglesDelta;
pub mod ExtendedRSSI;
pub mod FineOil;
pub mod Fluorescein;
pub mod FluorescentDissolvedOrganicMatter;
pub mod FollowCommand;
pub mod FollowCommandState;
pub mod FollowPath;
pub mod FollowRefState;
pub mod FollowReference;
pub mod FollowTrajectory;
pub mod FuelLevel;
pub mod GetOperationalLimits;
pub mod Goto;
pub mod GpioState;
pub mod GpioStateGet;
pub mod GpioStateSet;
pub mod GpsFix;
pub mod GpsFixRejection;
pub mod GroundVelocity;
pub mod Heartbeat;
pub mod IdleManeuver;
pub mod IoEvent;
pub mod IridiumMsgRx;
pub mod IridiumMsgTx;
pub mod IridiumMsgTxExtended;
pub mod IridiumTxStatus;
pub mod Launch;
pub mod LblBeacon;
pub mod LblConfig;
pub mod LblEstimate;
pub mod LblRange;
pub mod LblRangeAcceptance;
pub mod LcdControl;
pub mod LedBrightness;
pub mod LogBookControl;
pub mod LogBookEntry;
pub mod LoggingControl;
pub mod Loiter;
pub mod MagneticField;
pub mod Magnetometer;
pub mod ManeuverControlState;
pub mod MessagePart;
pub mod MonitorEntityState;
pub mod MsgList;
pub mod NavigationData;
pub mod NavigationUncertainty;
pub mod OperationalLimits;
pub mod OpticalBackscatter;
pub mod PH;
pub mod PathControlState;
pub mod PathPoint;
pub mod Phycocyanin;
pub mod Phycoerythrin;
pub mod PlanControl;
pub mod PlanControlState;
pub mod PlanDB;
pub mod PlanDBInformation;
pub mod PlanDBState;
pub mod PlanGeneration;
pub mod PlanManeuver;
pub mod PlanSpecification;
pub mod PlanStatistics;
pub mod PlanTransition;
pub mod PlanVariable;
pub mod PolygonVertex;
pub mod PopEntityParameters;
pub mod PopUp;
pub mod PowerChannelControl;
pub mod PowerChannelState;
pub mod PowerOperation;
pub mod Pressure;
pub mod Pulse;
pub mod PulseDetectionControl;
pub mod PushEntityParameters;
pub mod QueryEntityActivationState;
pub mod QueryEntityInfo;
pub mod QueryEntityParameters;
pub mod QueryEntityState;
pub mod QueryLedBrightness;
pub mod QueryPowerChannelState;
pub mod RSSI;
pub mod Redox;
pub mod Reference;
pub mod RegisterManeuver;
pub mod RelativeHumidity;
pub mod RemoteActions;
pub mod RemoteActionsRequest;
pub mod ReplayControl;
pub mod ReportControl;
pub mod RestartSystem;
pub mod RhodamineDye;
pub mod Rows;
pub mod Rpm;
pub mod Salinity;
pub mod SaveEntityParameters;
pub mod ServoPosition;
pub mod SetEntityParameters;
pub mod SetLedBrightness;
pub mod SetServoPosition;
pub mod SetThrusterActuation;
pub mod SimulatedState;
pub mod Sms;
pub mod SmsRx;
pub mod SmsState;
pub mod SmsTx;
pub mod SonarData;
pub mod SonarPulse;
pub mod SoundSpeed;
pub mod StationKeeping;
pub mod StopManeuver;
pub mod StorageUsage;
pub mod Tachograph;
pub mod Teleoperation;
pub mod TeleoperationDone;
pub mod Temperature;
pub mod TextMessage;
pub mod TrajectoryPoint;
pub mod TransportBindings;
pub mod TrexAttribute;
pub mod TrexOperation;
pub mod TrexToken;
pub mod Turbidity;
pub mod UamRxFrame;
pub mod UamRxRange;
pub mod UamTxFrame;
pub mod UamTxRange;
pub mod UamTxStatus;
pub mod UsblAngles;
pub mod UsblAnglesExtended;
pub mod UsblConfig;
pub mod UsblFix;
pub mod UsblFixExtended;
pub mod UsblModem;
pub mod UsblPosition;
pub mod UsblPositionExtended;
pub mod VehicleCommand;
pub mod VehicleLinks;
pub mod VehicleMedium;
pub mod VehicleOperationalLimits;
pub mod VehicleState;
pub mod VelocityDelta;
pub mod Voltage;
pub mod WaterDensity;
pub mod WaterVelocity;
pub mod YoYo;

/// IMC Constants
pub const DUNE_IMC_VERSION: &'static str = "5.4.24";
pub const DUNE_IMC_CONST_SYNC: u16 = 0xFE54;
pub const DUNE_IMC_CONST_SYNC_REV: u16 = 0x54FE;
pub const IMC_CONST_HEADER_SIZE: u8 = 20;
pub const IMC_CONST_FOOTER_SIZE: u8 = 2;
pub const IMC_CONST_UNK_EID: u8 = 255;
pub const DUNE_IMC_CONST_NULL_ID: u16 = 65535;
pub const DUNE_IMC_CONST_MAX_SIZE: usize = 65535;
pub const DUNE_IMC_CONST_SYS_EID: u8 = 0;

/// Global Enumerators

/// Speed Units
#[allow(non_camel_case_types)]
pub enum SpeedUnits {
    /// Meters per second
    SUNITS_METERS_PS = 0,
    /// RPM
    SUNITS_RPM = 1,
    /// Percentage
    SUNITS_PERCENTAGE = 2,
}

/// System Type
#[allow(non_camel_case_types)]
pub enum SystemType {
    /// CCU
    SYSTEMTYPE_CCU = 0,
    /// Human-portable Sensor
    SYSTEMTYPE_HUMANSENSOR = 1,
    /// UUV
    SYSTEMTYPE_UUV = 2,
    /// USV
    SYSTEMTYPE_USV = 3,
    /// UAV
    SYSTEMTYPE_UAV = 4,
    /// UGV
    SYSTEMTYPE_UGV = 5,
    /// Static sensor
    SYSTEMTYPE_STATICSENSOR = 6,
    /// Mobile sensor
    SYSTEMTYPE_MOBILESENSOR = 7,
    /// Wireless Sensor Network
    SYSTEMTYPE_WSN = 8,
}

/// Z Units
#[allow(non_camel_case_types)]
pub enum ZUnits {
    /// None
    Z_NONE = 0,
    /// Depth
    Z_DEPTH = 1,
    /// Altitude
    Z_ALTITUDE = 2,
    /// Height
    Z_HEIGHT = 3,
}

/// RSSI Units
#[allow(non_camel_case_types)]
pub enum RSSIUnits {
    /// Decibel
    RSSIUNITS_dB = 0,
    /// Percentage
    RSSIUNITS_PERCENTAGE = 1,
}

/// Global Bitfields
/// Control Loops Mask
#[allow(non_camel_case_types)]
pub mod CLoopsMask {
    /// None
    pub const CL_NONE: u32 = 0x00000000;
    /// Path Control
    pub const CL_PATH: u32 = 0x00000001;
    /// Teleoperation Control
    pub const CL_TELEOPERATION: u32 = 0x00000002;
    /// Altitude Control
    pub const CL_ALTITUDE: u32 = 0x00000004;
    /// Depth Control
    pub const CL_DEPTH: u32 = 0x00000008;
    /// Roll Control
    pub const CL_ROLL: u32 = 0x00000010;
    /// Pitch Control
    pub const CL_PITCH: u32 = 0x00000020;
    /// Yaw Control
    pub const CL_YAW: u32 = 0x00000040;
    /// Speed Control
    pub const CL_SPEED: u32 = 0x00000080;
    /// Yaw Rate Control
    pub const CL_YAW_RATE: u32 = 0x00000100;
    /// Torque Control
    pub const CL_TORQUE: u32 = 0x00000400;
    /// Force Control
    pub const CL_FORCE: u32 = 0x00000800;
    /// Non-overridable control
    pub const CL_NO_OVERRIDE: u32 = 0x80000000;
    /// All
    pub const CL_ALL: u32 = 0xFFFFFFFF;
}

/// Operational Limits Mask
#[allow(non_camel_case_types)]
pub mod OpLimitsMask {
    /// Maximum Depth
    pub const OPL_MAX_DEPTH: u32 = 0x01;
    /// Minimum Altitude
    pub const OPL_MIN_ALT: u32 = 0x02;
    /// Maximum Altitude
    pub const OPL_MAX_ALT: u32 = 0x04;
    /// Minimum Speed
    pub const OPL_MIN_SPEED: u32 = 0x08;
    /// Maximum Speed
    pub const OPL_MAX_SPEED: u32 = 0x10;
    /// Maximum Vertical Rate
    pub const OPL_MAX_VRATE: u32 = 0x20;
    /// Operation Area
    pub const OPL_AREA: u32 = 0x40;
}

/// Helper enum to use inline messages
enum InlineMessage {
    ImcAbort(Abort::Abort),
    ImcAborted(Aborted::Aborted),
    ImcAcceleration(Acceleration::Acceleration),
    ImcAcousticLink(AcousticLink::AcousticLink),
    ImcAcousticOperation(AcousticOperation::AcousticOperation),
    ImcAcousticSystems(AcousticSystems::AcousticSystems),
    ImcAcousticSystemsQuery(AcousticSystemsQuery::AcousticSystemsQuery),
    ImcAirSaturation(AirSaturation::AirSaturation),
    ImcAlignment(Alignment::Alignment),
    ImcAlignmentState(AlignmentState::AlignmentState),
    ImcAllocatedControlTorques(AllocatedControlTorques::AllocatedControlTorques),
    ImcAngularVelocity(AngularVelocity::AngularVelocity),
    ImcAnnounce(Announce::Announce),
    ImcAnnounceService(AnnounceService::AnnounceService),
    ImcBeamConfig(BeamConfig::BeamConfig),
    ImcBrake(Brake::Brake),
    ImcButtonEvent(ButtonEvent::ButtonEvent),
    ImcCacheControl(CacheControl::CacheControl),
    ImcCalibration(Calibration::Calibration),
    ImcCcuEvent(CcuEvent::CcuEvent),
    ImcChlorophyll(Chlorophyll::Chlorophyll),
    ImcClockControl(ClockControl::ClockControl),
    ImcCollision(Collision::Collision),
    ImcColoredDissolvedOrganicMatter(ColoredDissolvedOrganicMatter::ColoredDissolvedOrganicMatter),
    ImcCommand(Command::Command),
    ImcCompassCalibration(CompassCalibration::CompassCalibration),
    ImcConductivity(Conductivity::Conductivity),
    ImcControlLoops(ControlLoops::ControlLoops),
    ImcControlParcel(ControlParcel::ControlParcel),
    ImcCpuUsage(CpuUsage::CpuUsage),
    ImcCrudeOil(CrudeOil::CrudeOil),
    ImcCurrent(Current::Current),
    ImcCustomManeuver(CustomManeuver::CustomManeuver),
    ImcDataSanity(DataSanity::DataSanity),
    ImcDepth(Depth::Depth),
    ImcDesiredControl(DesiredControl::DesiredControl),
    ImcDesiredHeading(DesiredHeading::DesiredHeading),
    ImcDesiredHeadingRate(DesiredHeadingRate::DesiredHeadingRate),
    ImcDesiredPath(DesiredPath::DesiredPath),
    ImcDesiredPitch(DesiredPitch::DesiredPitch),
    ImcDesiredRoll(DesiredRoll::DesiredRoll),
    ImcDesiredSpeed(DesiredSpeed::DesiredSpeed),
    ImcDesiredZ(DesiredZ::DesiredZ),
    ImcDevDataBinary(DevDataBinary::DevDataBinary),
    ImcDevDataText(DevDataText::DevDataText),
    ImcDeviceState(DeviceState::DeviceState),
    ImcDislodge(Dislodge::Dislodge),
    ImcDissolvedOrganicMatter(DissolvedOrganicMatter::DissolvedOrganicMatter),
    ImcDissolvedOxygen(DissolvedOxygen::DissolvedOxygen),
    ImcDistance(Distance::Distance),
    ImcDvlRejection(DvlRejection::DvlRejection),
    ImcElevator(Elevator::Elevator),
    ImcEntityActivationState(EntityActivationState::EntityActivationState),
    ImcEntityInfo(EntityInfo::EntityInfo),
    ImcEntityList(EntityList::EntityList),
    ImcEntityMonitoringState(EntityMonitoringState::EntityMonitoringState),
    ImcEntityParameter(EntityParameter::EntityParameter),
    ImcEntityParameters(EntityParameters::EntityParameters),
    ImcEntityState(EntityState::EntityState),
    ImcEstimatedState(EstimatedState::EstimatedState),
    ImcEstimatedStreamVelocity(EstimatedStreamVelocity::EstimatedStreamVelocity),
    ImcEulerAngles(EulerAngles::EulerAngles),
    ImcEulerAnglesDelta(EulerAnglesDelta::EulerAnglesDelta),
    ImcExtendedRSSI(ExtendedRSSI::ExtendedRSSI),
    ImcFineOil(FineOil::FineOil),
    ImcFluorescein(Fluorescein::Fluorescein),
    ImcFluorescentDissolvedOrganicMatter(FluorescentDissolvedOrganicMatter::FluorescentDissolvedOrganicMatter),
    ImcFollowCommand(FollowCommand::FollowCommand),
    ImcFollowCommandState(FollowCommandState::FollowCommandState),
    ImcFollowPath(FollowPath::FollowPath),
    ImcFollowRefState(FollowRefState::FollowRefState),
    ImcFollowReference(FollowReference::FollowReference),
    ImcFollowTrajectory(FollowTrajectory::FollowTrajectory),
    ImcFuelLevel(FuelLevel::FuelLevel),
    ImcGetOperationalLimits(GetOperationalLimits::GetOperationalLimits),
    ImcGoto(Goto::Goto),
    ImcGpioState(GpioState::GpioState),
    ImcGpioStateGet(GpioStateGet::GpioStateGet),
    ImcGpioStateSet(GpioStateSet::GpioStateSet),
    ImcGpsFix(GpsFix::GpsFix),
    ImcGpsFixRejection(GpsFixRejection::GpsFixRejection),
    ImcGroundVelocity(GroundVelocity::GroundVelocity),
    ImcHeartbeat(Heartbeat::Heartbeat),
    ImcIdleManeuver(IdleManeuver::IdleManeuver),
    ImcIoEvent(IoEvent::IoEvent),
    ImcIridiumMsgRx(IridiumMsgRx::IridiumMsgRx),
    ImcIridiumMsgTx(IridiumMsgTx::IridiumMsgTx),
    ImcIridiumMsgTxExtended(IridiumMsgTxExtended::IridiumMsgTxExtended),
    ImcIridiumTxStatus(IridiumTxStatus::IridiumTxStatus),
    ImcLaunch(Launch::Launch),
    ImcLblBeacon(LblBeacon::LblBeacon),
    ImcLblConfig(LblConfig::LblConfig),
    ImcLblEstimate(LblEstimate::LblEstimate),
    ImcLblRange(LblRange::LblRange),
    ImcLblRangeAcceptance(LblRangeAcceptance::LblRangeAcceptance),
    ImcLcdControl(LcdControl::LcdControl),
    ImcLedBrightness(LedBrightness::LedBrightness),
    ImcLogBookControl(LogBookControl::LogBookControl),
    ImcLogBookEntry(LogBookEntry::LogBookEntry),
    ImcLoggingControl(LoggingControl::LoggingControl),
    ImcLoiter(Loiter::Loiter),
    ImcMagneticField(MagneticField::MagneticField),
    ImcMagnetometer(Magnetometer::Magnetometer),
    ImcManeuverControlState(ManeuverControlState::ManeuverControlState),
    ImcMessagePart(MessagePart::MessagePart),
    ImcMonitorEntityState(MonitorEntityState::MonitorEntityState),
    ImcMsgList(MsgList::MsgList),
    ImcNavigationData(NavigationData::NavigationData),
    ImcNavigationUncertainty(NavigationUncertainty::NavigationUncertainty),
    ImcOperationalLimits(OperationalLimits::OperationalLimits),
    ImcOpticalBackscatter(OpticalBackscatter::OpticalBackscatter),
    ImcPH(PH::PH),
    ImcPathControlState(PathControlState::PathControlState),
    ImcPathPoint(PathPoint::PathPoint),
    ImcPhycocyanin(Phycocyanin::Phycocyanin),
    ImcPhycoerythrin(Phycoerythrin::Phycoerythrin),
    ImcPlanControl(PlanControl::PlanControl),
    ImcPlanControlState(PlanControlState::PlanControlState),
    ImcPlanDB(PlanDB::PlanDB),
    ImcPlanDBInformation(PlanDBInformation::PlanDBInformation),
    ImcPlanDBState(PlanDBState::PlanDBState),
    ImcPlanGeneration(PlanGeneration::PlanGeneration),
    ImcPlanManeuver(PlanManeuver::PlanManeuver),
    ImcPlanSpecification(PlanSpecification::PlanSpecification),
    ImcPlanStatistics(PlanStatistics::PlanStatistics),
    ImcPlanTransition(PlanTransition::PlanTransition),
    ImcPlanVariable(PlanVariable::PlanVariable),
    ImcPolygonVertex(PolygonVertex::PolygonVertex),
    ImcPopEntityParameters(PopEntityParameters::PopEntityParameters),
    ImcPopUp(PopUp::PopUp),
    ImcPowerChannelControl(PowerChannelControl::PowerChannelControl),
    ImcPowerChannelState(PowerChannelState::PowerChannelState),
    ImcPowerOperation(PowerOperation::PowerOperation),
    ImcPressure(Pressure::Pressure),
    ImcPulse(Pulse::Pulse),
    ImcPulseDetectionControl(PulseDetectionControl::PulseDetectionControl),
    ImcPushEntityParameters(PushEntityParameters::PushEntityParameters),
    ImcQueryEntityActivationState(QueryEntityActivationState::QueryEntityActivationState),
    ImcQueryEntityInfo(QueryEntityInfo::QueryEntityInfo),
    ImcQueryEntityParameters(QueryEntityParameters::QueryEntityParameters),
    ImcQueryEntityState(QueryEntityState::QueryEntityState),
    ImcQueryLedBrightness(QueryLedBrightness::QueryLedBrightness),
    ImcQueryPowerChannelState(QueryPowerChannelState::QueryPowerChannelState),
    ImcRSSI(RSSI::RSSI),
    ImcRedox(Redox::Redox),
    ImcReference(Reference::Reference),
    ImcRegisterManeuver(RegisterManeuver::RegisterManeuver),
    ImcRelativeHumidity(RelativeHumidity::RelativeHumidity),
    ImcRemoteActions(RemoteActions::RemoteActions),
    ImcRemoteActionsRequest(RemoteActionsRequest::RemoteActionsRequest),
    ImcReplayControl(ReplayControl::ReplayControl),
    ImcReportControl(ReportControl::ReportControl),
    ImcRestartSystem(RestartSystem::RestartSystem),
    ImcRhodamineDye(RhodamineDye::RhodamineDye),
    ImcRows(Rows::Rows),
    ImcRpm(Rpm::Rpm),
    ImcSalinity(Salinity::Salinity),
    ImcSaveEntityParameters(SaveEntityParameters::SaveEntityParameters),
    ImcServoPosition(ServoPosition::ServoPosition),
    ImcSetEntityParameters(SetEntityParameters::SetEntityParameters),
    ImcSetLedBrightness(SetLedBrightness::SetLedBrightness),
    ImcSetServoPosition(SetServoPosition::SetServoPosition),
    ImcSetThrusterActuation(SetThrusterActuation::SetThrusterActuation),
    ImcSimulatedState(SimulatedState::SimulatedState),
    ImcSms(Sms::Sms),
    ImcSmsRx(SmsRx::SmsRx),
    ImcSmsState(SmsState::SmsState),
    ImcSmsTx(SmsTx::SmsTx),
    ImcSonarData(SonarData::SonarData),
    ImcSonarPulse(SonarPulse::SonarPulse),
    ImcSoundSpeed(SoundSpeed::SoundSpeed),
    ImcStationKeeping(StationKeeping::StationKeeping),
    ImcStopManeuver(StopManeuver::StopManeuver),
    ImcStorageUsage(StorageUsage::StorageUsage),
    ImcTachograph(Tachograph::Tachograph),
    ImcTeleoperation(Teleoperation::Teleoperation),
    ImcTeleoperationDone(TeleoperationDone::TeleoperationDone),
    ImcTemperature(Temperature::Temperature),
    ImcTextMessage(TextMessage::TextMessage),
    ImcTrajectoryPoint(TrajectoryPoint::TrajectoryPoint),
    ImcTransportBindings(TransportBindings::TransportBindings),
    ImcTrexAttribute(TrexAttribute::TrexAttribute),
    ImcTrexOperation(TrexOperation::TrexOperation),
    ImcTrexToken(TrexToken::TrexToken),
    ImcTurbidity(Turbidity::Turbidity),
    ImcUamRxFrame(UamRxFrame::UamRxFrame),
    ImcUamRxRange(UamRxRange::UamRxRange),
    ImcUamTxFrame(UamTxFrame::UamTxFrame),
    ImcUamTxRange(UamTxRange::UamTxRange),
    ImcUamTxStatus(UamTxStatus::UamTxStatus),
    ImcUsblAngles(UsblAngles::UsblAngles),
    ImcUsblAnglesExtended(UsblAnglesExtended::UsblAnglesExtended),
    ImcUsblConfig(UsblConfig::UsblConfig),
    ImcUsblFix(UsblFix::UsblFix),
    ImcUsblFixExtended(UsblFixExtended::UsblFixExtended),
    ImcUsblModem(UsblModem::UsblModem),
    ImcUsblPosition(UsblPosition::UsblPosition),
    ImcUsblPositionExtended(UsblPositionExtended::UsblPositionExtended),
    ImcVehicleCommand(VehicleCommand::VehicleCommand),
    ImcVehicleLinks(VehicleLinks::VehicleLinks),
    ImcVehicleMedium(VehicleMedium::VehicleMedium),
    ImcVehicleOperationalLimits(VehicleOperationalLimits::VehicleOperationalLimits),
    ImcVehicleState(VehicleState::VehicleState),
    ImcVelocityDelta(VelocityDelta::VelocityDelta),
    ImcVoltage(Voltage::Voltage),
    ImcWaterDensity(WaterDensity::WaterDensity),
    ImcWaterVelocity(WaterVelocity::WaterVelocity),
    ImcYoYo(YoYo::YoYo),
}
