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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

/// Base
pub mod Header;
pub mod Message;
pub mod MessageGroup;
pub mod factory;
pub mod packet;
pub type MessageList<T> = Vec<T>;

pub mod ADCPBeam;
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
pub mod AssetReport;
pub mod BeamConfig;
pub mod Brake;
pub mod ButtonEvent;
pub mod CacheControl;
pub mod Calibration;
pub mod CcuEvent;
pub mod ChargingState;
pub mod Chlorophyll;
pub mod ClockControl;
pub mod Collision;
pub mod ColoredDissolvedOrganicMatter;
pub mod CommRestriction;
pub mod Command;
pub mod CompassCalibration;
pub mod Conductivity;
pub mod ControlLoops;
pub mod ControlParcel;
pub mod CpuUsage;
pub mod CrudeOil;
pub mod Current;
pub mod CurrentProfile;
pub mod CurrentProfileCell;
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
pub mod MemUsage;
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
pub mod SmsRequest;
pub mod SmsRx;
pub mod SmsState;
pub mod SmsStatus;
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
pub mod TotalMagIntensity;
pub mod TrajectoryPoint;
pub mod TransportBindings;
pub mod TrexAttribute;
pub mod TrexOperation;
pub mod TrexToken;
pub mod Turbidity;
pub mod TypedEntityParameter;
pub mod TypedEntityParameters;
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
pub mod ValidatePlan;
pub mod ValuesIf;
pub mod VehicleCommand;
pub mod VehicleLinks;
pub mod VehicleMedium;
pub mod VehicleOperationalLimits;
pub mod VehicleState;
pub mod VelocityDelta;
pub mod Voltage;
pub mod WaterDensity;
pub mod WaterVelocity;
pub mod WifiNetwork;
pub mod WifiStats;
pub mod YoYo;

/// IMC Constants
pub const DUNE_IMC_VERSION: &str = "5.4.25";
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
