use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

pub enum TypeEnum {
    // Before Plan
    TP_PREPLAN = 0,
    // During Plan
    TP_INPLAN = 1,
    // After Plan
    TP_POSTPLAN = 2,
}

impl TypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            TP_PREPLAN => 0,
            TP_INPLAN => 1,
            TP_POSTPLAN => 2,
        }
    }
}

pub enum PropertiesEnum {
    // Basic Plan
    PRP_BASIC = 0x00,
    // Nonlinear
    PRP_NONLINEAR = 0x01,
    // Infinite
    PRP_INFINITE = 0x02,
    // Cyclical
    PRP_CYCLICAL = 0x04,
    // All
    PRP_ALL = 0x07,
}

impl PropertiesEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            PRP_BASIC => 0x00,
            PRP_NONLINEAR => 0x01,
            PRP_INFINITE => 0x02,
            PRP_CYCLICAL => 0x04,
            PRP_ALL => 0x07,
        }
    }
}

/// Plan is cyclical.
pub struct PlanStatistics {
    /// IMC Header
    pub header: Header,

    /// The name of the plan to be generated.
    pub _plan_id: String,

    /// Type of plan statistics, if they are launched before, during or after the plan execution.
    pub _type: u8,

    /// All properties checked.
    pub _properties: u8,

    /// Maneuver and plan duration statistics in seconds, for example: Total=1000,Goto1=20,Rows=980
    pub _durations: String,

    /// Distances travelled in meters in each maneuver and/or total: Total=2000,Rows=1800,Elevator=200
    pub _distances: String,

    /// List of components active by plan actions during the plan and time active in seconds: Sidescan=100,Camera Module=150
    pub _actions: String,

    /// Amount of fuel spent, in battery percentage, by different parcels (if applicable): Total=35,Hotel=5,Payload=10,Motion=20,IMU=0
    pub _fuel: String,
}

impl PlanStatistics {
    pub fn new() -> PlanStatistics {
        let mut msg = PlanStatistics {
            header: Header::new(564),

            _plan_id: Default::default(),
            _type: Default::default(),
            _properties: Default::default(),
            _durations: Default::default(),
            _distances: Default::default(),
            _actions: Default::default(),
            _fuel: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PlanStatistics {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        564
    }

    fn clear(&mut self) {
        self.header.clear();

        self._plan_id = Default::default();

        self._type = Default::default();

        self._properties = Default::default();

        self._durations = Default::default();

        self._distances = Default::default();

        self._actions = Default::default();

        self._fuel = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._plan_id.len() + 2;

        dyn_size += self._durations.len() + 2;

        dyn_size += self._distances.len() + 2;

        dyn_size += self._actions.len() + 2;

        dyn_size += self._fuel.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_u8(self._type);
        bfr.put_u8(self._properties);
        serialize_bytes!(bfr, self._durations.as_bytes());
        serialize_bytes!(bfr, self._distances.as_bytes());
        serialize_bytes!(bfr, self._actions.as_bytes());
        serialize_bytes!(bfr, self._fuel.as_bytes());

        serialize_footer(bfr);
    }
}
