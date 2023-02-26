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
pub mod AcousticMessage;
pub mod AcousticOperation;
pub mod AcousticRelease;
pub mod AcousticRequest;
pub mod AcousticStatus;
pub mod AcousticSystems;
pub mod AcousticSystemsQuery;
pub mod AirSaturation;
pub mod Airflow;
pub mod Alignment;
pub mod AlignmentState;
pub mod AllocatedControlTorques;
pub mod AngularVelocity;
pub mod Announce;
pub mod AnnounceService;
pub mod ApmStatus;
pub mod ArmingState;
pub mod AssetReport;
pub mod AutonomousSection;
pub mod AutopilotMode;
pub mod BeamConfig;
pub mod Brake;
pub mod ButtonEvent;
pub mod CacheControl;
pub mod Calibration;
pub mod CameraZoom;
pub mod CcuEvent;
pub mod Chlorophyll;
pub mod ClockControl;
pub mod CloseSession;
pub mod Collision;
pub mod ColoredDissolvedOrganicMatter;
pub mod CommRestriction;
pub mod CommSystemsQuery;
pub mod CommsRelay;
pub mod CompassCalibration;
pub mod CompressedHistory;
pub mod CompressedImage;
pub mod Conductivity;
pub mod ControlLoops;
pub mod ControlParcel;
pub mod CoverArea;
pub mod CpuUsage;
pub mod CreateSession;
pub mod CrudeOil;
pub mod Current;
pub mod CurrentProfile;
pub mod CurrentProfileCell;
pub mod CustomManeuver;
pub mod DataSanity;
pub mod Depth;
pub mod DepthOffset;
pub mod DesiredControl;
pub mod DesiredHeading;
pub mod DesiredHeadingRate;
pub mod DesiredLinearState;
pub mod DesiredPath;
pub mod DesiredPitch;
pub mod DesiredRoll;
pub mod DesiredSpeed;
pub mod DesiredThrottle;
pub mod DesiredVelocity;
pub mod DesiredVerticalRate;
pub mod DesiredZ;
pub mod DevCalibrationControl;
pub mod DevCalibrationState;
pub mod DevDataBinary;
pub mod DevDataText;
pub mod DeviceState;
pub mod Dislodge;
pub mod DissolvedOrganicMatter;
pub mod DissolvedOxygen;
pub mod Distance;
pub mod DmsDetection;
pub mod Drop;
pub mod DvlRejection;
pub mod DynamicsSimParam;
pub mod Elevator;
pub mod EmergencyControl;
pub mod EmergencyControlState;
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
pub mod Event;
pub mod ExtendedRSSI;
pub mod ExternalNavData;
pub mod FineOil;
pub mod FlightEvent;
pub mod Fluorescein;
pub mod FluorescentDissolvedOrganicMatter;
pub mod FollowPath;
pub mod FollowPoint;
pub mod FollowRefState;
pub mod FollowReference;
pub mod FollowSystem;
pub mod FollowTrajectory;
pub mod Force;
pub mod FormCtrlParam;
pub mod FormState;
pub mod Formation;
pub mod FormationControlParams;
pub mod FormationEval;
pub mod FormationEvaluation;
pub mod FormationMonitor;
pub mod FormationParameters;
pub mod FormationPlanExecution;
pub mod FormationState;
pub mod FuelLevel;
pub mod GetImageCoords;
pub mod GetOperationalLimits;
pub mod GetParametersXml;
pub mod GetWorldCoordinates;
pub mod Goto;
pub mod GpioState;
pub mod GpioStateGet;
pub mod GpioStateSet;
pub mod GpsFix;
pub mod GpsFixRejection;
pub mod GpsFixRtk;
pub mod GpsNavData;
pub mod GroundVelocity;
pub mod GroupMembershipState;
pub mod GroupStreamVelocity;
pub mod Heartbeat;
pub mod HistoricCTD;
pub mod HistoricData;
pub mod HistoricDataQuery;
pub mod HistoricEvent;
pub mod HistoricSample;
pub mod HistoricSonarData;
pub mod HistoricTelemetry;
pub mod HomePosition;
pub mod IdleManeuver;
pub mod ImageTracking;
pub mod ImageTxSettings;
pub mod IndicatedSpeed;
pub mod IoEvent;
pub mod IridiumMsgRx;
pub mod IridiumMsgTx;
pub mod IridiumTxStatus;
pub mod Land;
pub mod Launch;
pub mod LblBeacon;
pub mod LblConfig;
pub mod LblEstimate;
pub mod LblRange;
pub mod LblRangeAcceptance;
pub mod LcdControl;
pub mod LeaderState;
pub mod LeakSimulation;
pub mod LedBrightness;
pub mod LinkLatency;
pub mod LinkLevel;
pub mod LogBookControl;
pub mod LogBookEntry;
pub mod LoggingControl;
pub mod Loiter;
pub mod LowLevelControl;
pub mod MagneticField;
pub mod Magnetometer;
pub mod ManeuverControlState;
pub mod ManeuverDone;
pub mod Map;
pub mod MapFeature;
pub mod MapPoint;
pub mod MessagePart;
pub mod MonitorEntityState;
pub mod MsgList;
pub mod NavigationData;
pub mod NavigationUncertainty;
pub mod NeptusBlob;
pub mod OperationalLimits;
pub mod OpticalBackscatter;
pub mod PH;
pub mod PWM;
pub mod ParametersXml;
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
pub mod ProfileSample;
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
pub mod RelativeState;
pub mod RemoteActions;
pub mod RemoteActionsRequest;
pub mod RemoteCommand;
pub mod RemoteSensorInfo;
pub mod RemoteState;
pub mod ReplayControl;
pub mod ReportControl;
pub mod ReportedState;
pub mod RestartSystem;
pub mod RhodamineDye;
pub mod Rows;
pub mod RowsCoverage;
pub mod Rpm;
pub mod SadcReadings;
pub mod Salinity;
pub mod Sample;
pub mod SaveEntityParameters;
pub mod Scalar;
pub mod ScheduledGoto;
pub mod ServoPosition;
pub mod SessionKeepAlive;
pub mod SessionStatus;
pub mod SessionSubscription;
pub mod SetControlSurfaceDeflection;
pub mod SetEntityParameters;
pub mod SetImageCoords;
pub mod SetLedBrightness;
pub mod SetPWM;
pub mod SetServoPosition;
pub mod SetThrusterActuation;
pub mod SimAcousticMessage;
pub mod SimulatedState;
pub mod Sms;
pub mod SmsRequest;
pub mod SmsRx;
pub mod SmsState;
pub mod SmsStatus;
pub mod SmsTx;
pub mod SoiCommand;
pub mod SoiPlan;
pub mod SoiState;
pub mod SoiWaypoint;
pub mod SonarData;
pub mod SoundSpeed;
pub mod StateReport;
pub mod StationKeeping;
pub mod StationKeepingExtended;
pub mod StopManeuver;
pub mod StorageUsage;
pub mod SystemGroup;
pub mod TCPRequest;
pub mod TCPStatus;
pub mod Tachograph;
pub mod Takeoff;
pub mod Target;
pub mod TelemetryMsg;
pub mod Teleoperation;
pub mod TeleoperationDone;
pub mod Temperature;
pub mod TextMessage;
pub mod Throttle;
pub mod TotalMagIntensity;
pub mod TrajectoryPoint;
pub mod TransmissionRequest;
pub mod TransmissionStatus;
pub mod TransportBindings;
pub mod TrexAttribute;
pub mod TrexCommand;
pub mod TrexObservation;
pub mod TrexOperation;
pub mod TrexPlan;
pub mod TrexToken;
pub mod TrueSpeed;
pub mod Turbidity;
pub mod UASimulation;
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
pub mod VSWR;
pub mod VehicleCommand;
pub mod VehicleFormation;
pub mod VehicleFormationParticipant;
pub mod VehicleLinks;
pub mod VehicleMedium;
pub mod VehicleOperationalLimits;
pub mod VehicleState;
pub mod VelocityDelta;
pub mod VerticalProfile;
pub mod Voltage;
pub mod VtolState;
pub mod WaterDensity;
pub mod WaterVelocity;
pub mod WindSpeed;
pub mod YoYo;

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
