//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Before Plan
    TP_PREPLAN = 0,
    /// During Plan
    TP_INPLAN = 1,
    /// After Plan
    TP_POSTPLAN = 2,
}

/// Properties
#[allow(non_camel_case_types)]
pub mod PropertiesBits {
    /// Basic Plan
    pub const PRP_BASIC: u32 = 0x00;
    /// Nonlinear
    pub const PRP_NONLINEAR: u32 = 0x01;
    /// Infinite
    pub const PRP_INFINITE: u32 = 0x02;
    /// Cyclical
    pub const PRP_CYCLICAL: u32 = 0x04;
    /// All
    pub const PRP_ALL: u32 = 0x07;
}

#[derive(Default)]
pub struct PlanStatistics {
    /// Message Header
    pub _header: Header,
    /// The name of the plan to be generated.
    pub _plan_id: String,
    /// Type of plan statistics, if they are launched before, during or after the plan execution.
    pub _type: u8,
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

impl Message for PlanStatistics {
    fn new() -> PlanStatistics {
        let msg = PlanStatistics {
            _header: Header::new(564),
            _plan_id: Default::default(),
            _type: Default::default(),
            _properties: Default::default(),
            _durations: Default::default(),
            _distances: Default::default(),
            _actions: Default::default(),
            _fuel: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        564
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        564
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(564);
        self._plan_id = Default::default();
        self._type = Default::default();
        self._properties = Default::default();
        self._durations = Default::default();
        self._distances = Default::default();
        self._actions = Default::default();
        self._fuel = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._plan_id.len() + 2;
        dyn_size += self._durations.len() + 2;
        dyn_size += self._distances.len() + 2;
        dyn_size += self._actions.len() + 2;
        dyn_size += self._fuel.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_u8(self._type);
        bfr.put_u8(self._properties);
        serialize_bytes!(bfr, self._durations.as_bytes());
        serialize_bytes!(bfr, self._distances.as_bytes());
        serialize_bytes!(bfr, self._actions.as_bytes());
        serialize_bytes!(bfr, self._fuel.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._plan_id);
        self._type = bfr.get_u8();
        self._properties = bfr.get_u8();
        deserialize_string!(bfr, self._durations);
        deserialize_string!(bfr, self._distances);
        deserialize_string!(bfr, self._actions);
        deserialize_string!(bfr, self._fuel);
        Ok(())
    }
}
