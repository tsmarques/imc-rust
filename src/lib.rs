#![allow(non_snake_case)]
#![allow(dead_code)]

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
// IMC XML MD5: b521199aa61f91939b6b6ed9e44d149b                            *
//###########################################################################

pub mod messages;

/// Base
pub mod MessageGroup;
pub mod factory;
pub mod packet;
pub type MessageList<T> = Vec<T>;

pub use messages::ADCPBeam::ADCPBeam;
pub use messages::Abort::Abort;
pub use messages::Aborted::Aborted;
pub use messages::Acceleration::Acceleration;
pub use messages::AcousticLink::AcousticLink;
pub use messages::AcousticMessage::AcousticMessage;
pub use messages::AcousticOperation::AcousticOperation;
pub use messages::AcousticRelease::AcousticRelease;
pub use messages::AcousticRequest::AcousticRequest;
pub use messages::AcousticStatus::AcousticStatus;
pub use messages::AcousticSystems::AcousticSystems;
pub use messages::AcousticSystemsQuery::AcousticSystemsQuery;
pub use messages::AirSaturation::AirSaturation;
pub use messages::Airflow::Airflow;
pub use messages::Alignment::Alignment;
pub use messages::AlignmentState::AlignmentState;
pub use messages::AllocatedControlTorques::AllocatedControlTorques;
pub use messages::AngularVelocity::AngularVelocity;
pub use messages::Announce::Announce;
pub use messages::AnnounceService::AnnounceService;
pub use messages::ApmStatus::ApmStatus;
pub use messages::ArmingState::ArmingState;
pub use messages::AssetReport::AssetReport;
pub use messages::AutonomousSection::AutonomousSection;
pub use messages::AutopilotMode::AutopilotMode;
pub use messages::BeamConfig::BeamConfig;
pub use messages::Brake::Brake;
pub use messages::ButtonEvent::ButtonEvent;
pub use messages::CacheControl::CacheControl;
pub use messages::Calibration::Calibration;
pub use messages::CameraZoom::CameraZoom;
pub use messages::CcuEvent::CcuEvent;
pub use messages::Chlorophyll::Chlorophyll;
pub use messages::ClockControl::ClockControl;
pub use messages::CloseSession::CloseSession;
pub use messages::Collision::Collision;
pub use messages::ColoredDissolvedOrganicMatter::ColoredDissolvedOrganicMatter;
pub use messages::CommRestriction::CommRestriction;
pub use messages::CommSystemsQuery::CommSystemsQuery;
pub use messages::CommsRelay::CommsRelay;
pub use messages::CompassCalibration::CompassCalibration;
pub use messages::CompressedHistory::CompressedHistory;
pub use messages::CompressedImage::CompressedImage;
pub use messages::Conductivity::Conductivity;
pub use messages::ControlLoops::ControlLoops;
pub use messages::ControlParcel::ControlParcel;
pub use messages::CoverArea::CoverArea;
pub use messages::CpuUsage::CpuUsage;
pub use messages::CreateSession::CreateSession;
pub use messages::CrudeOil::CrudeOil;
pub use messages::Current::Current;
pub use messages::CurrentProfile::CurrentProfile;
pub use messages::CurrentProfileCell::CurrentProfileCell;
pub use messages::CustomManeuver::CustomManeuver;
pub use messages::DataSanity::DataSanity;
pub use messages::Depth::Depth;
pub use messages::DepthOffset::DepthOffset;
pub use messages::DesiredControl::DesiredControl;
pub use messages::DesiredHeading::DesiredHeading;
pub use messages::DesiredHeadingRate::DesiredHeadingRate;
pub use messages::DesiredLinearState::DesiredLinearState;
pub use messages::DesiredPath::DesiredPath;
pub use messages::DesiredPitch::DesiredPitch;
pub use messages::DesiredRoll::DesiredRoll;
pub use messages::DesiredSpeed::DesiredSpeed;
pub use messages::DesiredThrottle::DesiredThrottle;
pub use messages::DesiredVelocity::DesiredVelocity;
pub use messages::DesiredVerticalRate::DesiredVerticalRate;
pub use messages::DesiredZ::DesiredZ;
pub use messages::DevCalibrationControl::DevCalibrationControl;
pub use messages::DevCalibrationState::DevCalibrationState;
pub use messages::DevDataBinary::DevDataBinary;
pub use messages::DevDataText::DevDataText;
pub use messages::DeviceState::DeviceState;
pub use messages::Dislodge::Dislodge;
pub use messages::DissolvedOrganicMatter::DissolvedOrganicMatter;
pub use messages::DissolvedOxygen::DissolvedOxygen;
pub use messages::Distance::Distance;
pub use messages::DmsDetection::DmsDetection;
pub use messages::Drop::Drop;
pub use messages::DvlRejection::DvlRejection;
pub use messages::DynamicsSimParam::DynamicsSimParam;
pub use messages::Elevator::Elevator;
pub use messages::EmergencyControl::EmergencyControl;
pub use messages::EmergencyControlState::EmergencyControlState;
pub use messages::EntityActivationState::EntityActivationState;
pub use messages::EntityInfo::EntityInfo;
pub use messages::EntityList::EntityList;
pub use messages::EntityMonitoringState::EntityMonitoringState;
pub use messages::EntityParameter::EntityParameter;
pub use messages::EntityParameters::EntityParameters;
pub use messages::EntityState::EntityState;
pub use messages::EstimatedState::EstimatedState;
pub use messages::EstimatedStreamVelocity::EstimatedStreamVelocity;
pub use messages::EulerAngles::EulerAngles;
pub use messages::EulerAnglesDelta::EulerAnglesDelta;
pub use messages::Event::Event;
pub use messages::ExtendedRSSI::ExtendedRSSI;
pub use messages::ExternalNavData::ExternalNavData;
pub use messages::FineOil::FineOil;
pub use messages::FlightEvent::FlightEvent;
pub use messages::Fluorescein::Fluorescein;
pub use messages::FluorescentDissolvedOrganicMatter::FluorescentDissolvedOrganicMatter;
pub use messages::FollowPath::FollowPath;
pub use messages::FollowPoint::FollowPoint;
pub use messages::FollowRefState::FollowRefState;
pub use messages::FollowReference::FollowReference;
pub use messages::FollowSystem::FollowSystem;
pub use messages::FollowTrajectory::FollowTrajectory;
pub use messages::Force::Force;
pub use messages::FormCtrlParam::FormCtrlParam;
pub use messages::FormState::FormState;
pub use messages::Formation::Formation;
pub use messages::FormationControlParams::FormationControlParams;
pub use messages::FormationEval::FormationEval;
pub use messages::FormationEvaluation::FormationEvaluation;
pub use messages::FormationMonitor::FormationMonitor;
pub use messages::FormationParameters::FormationParameters;
pub use messages::FormationPlanExecution::FormationPlanExecution;
pub use messages::FormationState::FormationState;
pub use messages::FuelLevel::FuelLevel;
pub use messages::GetImageCoords::GetImageCoords;
pub use messages::GetOperationalLimits::GetOperationalLimits;
pub use messages::GetParametersXml::GetParametersXml;
pub use messages::GetWorldCoordinates::GetWorldCoordinates;
pub use messages::Goto::Goto;
pub use messages::GpioState::GpioState;
pub use messages::GpioStateGet::GpioStateGet;
pub use messages::GpioStateSet::GpioStateSet;
pub use messages::GpsFix::GpsFix;
pub use messages::GpsFixRejection::GpsFixRejection;
pub use messages::GpsFixRtk::GpsFixRtk;
pub use messages::GpsNavData::GpsNavData;
pub use messages::GroundVelocity::GroundVelocity;
pub use messages::GroupMembershipState::GroupMembershipState;
pub use messages::GroupStreamVelocity::GroupStreamVelocity;
pub use messages::Header::Header;
pub use messages::Heartbeat::Heartbeat;
pub use messages::HistoricCTD::HistoricCTD;
pub use messages::HistoricData::HistoricData;
pub use messages::HistoricDataQuery::HistoricDataQuery;
pub use messages::HistoricEvent::HistoricEvent;
pub use messages::HistoricSample::HistoricSample;
pub use messages::HistoricSonarData::HistoricSonarData;
pub use messages::HistoricTelemetry::HistoricTelemetry;
pub use messages::HomePosition::HomePosition;
pub use messages::IdleManeuver::IdleManeuver;
pub use messages::ImageTracking::ImageTracking;
pub use messages::ImageTxSettings::ImageTxSettings;
pub use messages::IndicatedSpeed::IndicatedSpeed;
pub use messages::IoEvent::IoEvent;
pub use messages::IridiumMsgRx::IridiumMsgRx;
pub use messages::IridiumMsgTx::IridiumMsgTx;
pub use messages::IridiumTxStatus::IridiumTxStatus;
pub use messages::Land::Land;
pub use messages::Launch::Launch;
pub use messages::LblBeacon::LblBeacon;
pub use messages::LblConfig::LblConfig;
pub use messages::LblEstimate::LblEstimate;
pub use messages::LblRange::LblRange;
pub use messages::LblRangeAcceptance::LblRangeAcceptance;
pub use messages::LcdControl::LcdControl;
pub use messages::LeaderState::LeaderState;
pub use messages::LeakSimulation::LeakSimulation;
pub use messages::LedBrightness::LedBrightness;
pub use messages::LinkLatency::LinkLatency;
pub use messages::LinkLevel::LinkLevel;
pub use messages::LogBookControl::LogBookControl;
pub use messages::LogBookEntry::LogBookEntry;
pub use messages::LoggingControl::LoggingControl;
pub use messages::Loiter::Loiter;
pub use messages::LowLevelControl::LowLevelControl;
pub use messages::MagneticField::MagneticField;
pub use messages::Magnetometer::Magnetometer;
pub use messages::ManeuverControlState::ManeuverControlState;
pub use messages::ManeuverDone::ManeuverDone;
pub use messages::Map::Map;
pub use messages::MapFeature::MapFeature;
pub use messages::MapPoint::MapPoint;
pub use messages::Message::Message;
pub use messages::MessagePart::MessagePart;
pub use messages::MonitorEntityState::MonitorEntityState;
pub use messages::MsgList::MsgList;
pub use messages::NavigationData::NavigationData;
pub use messages::NavigationUncertainty::NavigationUncertainty;
pub use messages::NeptusBlob::NeptusBlob;
pub use messages::OperationalLimits::OperationalLimits;
pub use messages::OpticalBackscatter::OpticalBackscatter;
pub use messages::ParametersXml::ParametersXml;
pub use messages::PathControlState::PathControlState;
pub use messages::PathPoint::PathPoint;
pub use messages::Phycocyanin::Phycocyanin;
pub use messages::Phycoerythrin::Phycoerythrin;
pub use messages::PlanControl::PlanControl;
pub use messages::PlanControlState::PlanControlState;
pub use messages::PlanDB::PlanDB;
pub use messages::PlanDBInformation::PlanDBInformation;
pub use messages::PlanDBState::PlanDBState;
pub use messages::PlanGeneration::PlanGeneration;
pub use messages::PlanManeuver::PlanManeuver;
pub use messages::PlanSpecification::PlanSpecification;
pub use messages::PlanStatistics::PlanStatistics;
pub use messages::PlanTransition::PlanTransition;
pub use messages::PlanVariable::PlanVariable;
pub use messages::PolygonVertex::PolygonVertex;
pub use messages::PopEntityParameters::PopEntityParameters;
pub use messages::PopUp::PopUp;
pub use messages::PowerChannelControl::PowerChannelControl;
pub use messages::PowerChannelState::PowerChannelState;
pub use messages::PowerOperation::PowerOperation;
pub use messages::Pressure::Pressure;
pub use messages::ProfileSample::ProfileSample;
pub use messages::Pulse::Pulse;
pub use messages::PulseDetectionControl::PulseDetectionControl;
pub use messages::PushEntityParameters::PushEntityParameters;
pub use messages::QueryEntityActivationState::QueryEntityActivationState;
pub use messages::QueryEntityInfo::QueryEntityInfo;
pub use messages::QueryEntityParameters::QueryEntityParameters;
pub use messages::QueryEntityState::QueryEntityState;
pub use messages::QueryLedBrightness::QueryLedBrightness;
pub use messages::QueryPowerChannelState::QueryPowerChannelState;
pub use messages::Redox::Redox;
pub use messages::Reference::Reference;
pub use messages::RegisterManeuver::RegisterManeuver;
pub use messages::RelativeHumidity::RelativeHumidity;
pub use messages::RelativeState::RelativeState;
pub use messages::RemoteActions::RemoteActions;
pub use messages::RemoteActionsRequest::RemoteActionsRequest;
pub use messages::RemoteCommand::RemoteCommand;
pub use messages::RemoteSensorInfo::RemoteSensorInfo;
pub use messages::RemoteState::RemoteState;
pub use messages::ReplayControl::ReplayControl;
pub use messages::ReportControl::ReportControl;
pub use messages::ReportedState::ReportedState;
pub use messages::RestartSystem::RestartSystem;
pub use messages::RhodamineDye::RhodamineDye;
pub use messages::Rows::Rows;
pub use messages::RowsCoverage::RowsCoverage;
pub use messages::Rpm::Rpm;
pub use messages::SadcReadings::SadcReadings;
pub use messages::Salinity::Salinity;
pub use messages::Sample::Sample;
pub use messages::SaveEntityParameters::SaveEntityParameters;
pub use messages::Scalar::Scalar;
pub use messages::ScheduledGoto::ScheduledGoto;
pub use messages::ServoPosition::ServoPosition;
pub use messages::SessionKeepAlive::SessionKeepAlive;
pub use messages::SessionStatus::SessionStatus;
pub use messages::SessionSubscription::SessionSubscription;
pub use messages::SetControlSurfaceDeflection::SetControlSurfaceDeflection;
pub use messages::SetEntityParameters::SetEntityParameters;
pub use messages::SetImageCoords::SetImageCoords;
pub use messages::SetLedBrightness::SetLedBrightness;
pub use messages::SetPWM::SetPWM;
pub use messages::SetServoPosition::SetServoPosition;
pub use messages::SetThrusterActuation::SetThrusterActuation;
pub use messages::SimAcousticMessage::SimAcousticMessage;
pub use messages::SimulatedState::SimulatedState;
pub use messages::Sms::Sms;
pub use messages::SmsRequest::SmsRequest;
pub use messages::SmsRx::SmsRx;
pub use messages::SmsState::SmsState;
pub use messages::SmsStatus::SmsStatus;
pub use messages::SmsTx::SmsTx;
pub use messages::SoiCommand::SoiCommand;
pub use messages::SoiPlan::SoiPlan;
pub use messages::SoiState::SoiState;
pub use messages::SoiWaypoint::SoiWaypoint;
pub use messages::SonarData::SonarData;
pub use messages::SoundSpeed::SoundSpeed;
pub use messages::StateReport::StateReport;
pub use messages::StationKeeping::StationKeeping;
pub use messages::StationKeepingExtended::StationKeepingExtended;
pub use messages::StopManeuver::StopManeuver;
pub use messages::StorageUsage::StorageUsage;
pub use messages::SystemGroup::SystemGroup;
pub use messages::TCPRequest::TCPRequest;
pub use messages::TCPStatus::TCPStatus;
pub use messages::Tachograph::Tachograph;
pub use messages::Takeoff::Takeoff;
pub use messages::Target::Target;
pub use messages::TelemetryMsg::TelemetryMsg;
pub use messages::Teleoperation::Teleoperation;
pub use messages::TeleoperationDone::TeleoperationDone;
pub use messages::Temperature::Temperature;
pub use messages::TextMessage::TextMessage;
pub use messages::Throttle::Throttle;
pub use messages::TotalMagIntensity::TotalMagIntensity;
pub use messages::TrajectoryPoint::TrajectoryPoint;
pub use messages::TransmissionRequest::TransmissionRequest;
pub use messages::TransmissionStatus::TransmissionStatus;
pub use messages::TransportBindings::TransportBindings;
pub use messages::TrexAttribute::TrexAttribute;
pub use messages::TrexCommand::TrexCommand;
pub use messages::TrexObservation::TrexObservation;
pub use messages::TrexOperation::TrexOperation;
pub use messages::TrexPlan::TrexPlan;
pub use messages::TrexToken::TrexToken;
pub use messages::TrueSpeed::TrueSpeed;
pub use messages::Turbidity::Turbidity;
pub use messages::UASimulation::UASimulation;
pub use messages::UamRxFrame::UamRxFrame;
pub use messages::UamRxRange::UamRxRange;
pub use messages::UamTxFrame::UamTxFrame;
pub use messages::UamTxRange::UamTxRange;
pub use messages::UamTxStatus::UamTxStatus;
pub use messages::UsblAngles::UsblAngles;
pub use messages::UsblAnglesExtended::UsblAnglesExtended;
pub use messages::UsblConfig::UsblConfig;
pub use messages::UsblFix::UsblFix;
pub use messages::UsblFixExtended::UsblFixExtended;
pub use messages::UsblModem::UsblModem;
pub use messages::UsblPosition::UsblPosition;
pub use messages::UsblPositionExtended::UsblPositionExtended;
pub use messages::VehicleCommand::VehicleCommand;
pub use messages::VehicleFormation::VehicleFormation;
pub use messages::VehicleFormationParticipant::VehicleFormationParticipant;
pub use messages::VehicleLinks::VehicleLinks;
pub use messages::VehicleMedium::VehicleMedium;
pub use messages::VehicleOperationalLimits::VehicleOperationalLimits;
pub use messages::VehicleState::VehicleState;
pub use messages::VelocityDelta::VelocityDelta;
pub use messages::VerticalProfile::VerticalProfile;
pub use messages::Voltage::Voltage;
pub use messages::VtolState::VtolState;
pub use messages::WaterDensity::WaterDensity;
pub use messages::WaterVelocity::WaterVelocity;
pub use messages::WindSpeed::WindSpeed;
pub use messages::YoYo::YoYo;
pub use messages::PH::PH;
pub use messages::PWM::PWM;
pub use messages::RSSI::RSSI;
pub use messages::VSWR::VSWR;

/// IMC Constants
pub const DUNE_IMC_VERSION: &str = "5.4.30";
pub const DUNE_IMC_CONST_SYNC: u16 = 0xFE54;
pub const DUNE_IMC_CONST_SYNC_REV: u16 = 0x54FE;
pub const IMC_CONST_HEADER_SIZE: u8 = 20;
pub const IMC_CONST_FOOTER_SIZE: u8 = 2;
pub const IMC_CONST_UNK_EID: u8 = 255;
pub const DUNE_IMC_CONST_NULL_ID: u16 = 65535;
pub const DUNE_IMC_CONST_MAX_SIZE: usize = 65535;
pub const DUNE_IMC_CONST_SYS_EID: u8 = 0;

/// Global Enumerators

/// Boolean Value
#[allow(non_camel_case_types)]
pub enum Boolean {
    /// False
    BOOL_FALSE = 0,
    /// True
    BOOL_TRUE = 1,
}

/// Controlled Mode
#[allow(non_camel_case_types)]
pub enum ControlledMode {
    /// Relinquish / Handoff Control
    CTLMD_RELINQUISH_HANDOFF_CTL = 0,
    /// Request Control
    CTLMD_REQUEST_CTL = 1,
    /// Override Control
    CTLMD_OVERRIDE_CTL = 2,
}

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

/// UAV Type
#[allow(non_camel_case_types)]
pub enum UAVType {
    /// Fixed-Wing
    UAVTYPE_FIXEDWING = 0,
    /// Copter
    UAVTYPE_COPTER = 1,
    /// Vtol
    UAVTYPE_VTOL = 2,
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
    /// Vertical Rate Control
    pub const CL_VERTICAL_RATE: u32 = 0x00000200;
    /// Torque Control
    pub const CL_TORQUE: u32 = 0x00000400;
    /// Force Control
    pub const CL_FORCE: u32 = 0x00000800;
    /// Velocity Control
    pub const CL_VELOCITY: u32 = 0x00001000;
    /// Throttle Control
    pub const CL_THROTTLE: u32 = 0x00002000;
    /// Unspecified External Control
    pub const CL_EXTERNAL: u32 = 0x40000000;
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

pub fn is<T: Message>(id: u16) -> bool {
    T::static_id() == id
}

// FIXME do we really need 'static?
pub fn cast<T: 'static>(m: &dyn Message) -> Option<&T> {
    m.as_any().downcast_ref::<T>()
}
