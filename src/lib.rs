#![allow(non_snake_case)]

pub mod Header;
pub mod Message;
pub mod MessageGroup;

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

pub const DUNE_IMC_VERSION: &'static str = "5.4.24";

// IMC Synchronization number
pub const DUNE_IMC_CONST_SYNC: u16 = 0xFE54;

// Reversed synchronization number.
pub const DUNE_IMC_CONST_SYNC_REV: u16 = 0x54FE;

// IMC's header size
pub const IMC_CONST_HEADER_SIZE: u8 = 20;

// IMC's footer size
pub const IMC_CONST_FOOTER_SIZE: u8 = 2;

// Unknown entity identifier.
pub const IMC_CONST_UNK_EID: u8 = 255;

// Identification number of the null message.
pub const DUNE_IMC_CONST_NULL_ID: u16 = 65535;

// Maximum message data size.
pub const DUNE_IMC_CONST_MAX_SIZE: usize = 65535;

// System entity identifier.
pub const DUNE_IMC_CONST_SYS_EID: u8 = 0;

pub mod utils {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub(crate) fn get_timestamp_secs() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("failed to get timestamp")
            .as_secs_f64()
    }
}
