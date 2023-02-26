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

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;
use crate::packet::*;
use crate::EstimatedState::EstimatedState;
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Nav Data Type
#[allow(non_camel_case_types)]
pub enum NavDataTypeEnum {
    /// Full State
    EXTNAV_FULL = 0,
    /// Attitude Heading Reference System Only
    EXTNAV_AHRS = 1,
    /// Position Reference System only
    EXTNAV_POSREF = 2,
}

/// This message is a representation of the state of the vehicle,
/// as seen by an external navigation computer.
/// An example usage is when DUNE is used with ardupilot. The
/// data gathered from the autopilot is a complete navigation
/// solution.
/// ExternalNavData contains an inline Estimated State, which
/// is a complete description of the system
/// in terms of parameters such as position, orientation and
/// velocities at a particular moment in time.
/// The Type field selects wether the navigation data is a
/// full state estimation, or only concerns attitude or
/// position/velocity.
#[derive(Default, Clone)]
pub struct ExternalNavData {
    /// Message Header
    pub _header: Header,
    /// External Navigation Data.
    pub _state: Option<EstimatedState>,
    /// The type of external navigation data
    pub _type: u8,
}

impl Message for ExternalNavData {
    fn new() -> ExternalNavData {
        

        ExternalNavData {
            _header: Header::new(294),
            _state: Default::default(),
            _type: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        294
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        294
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        if let Some(m) = &mut self._state {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        if let Some(m) = &mut self._state {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        if let Some(m) = &mut self._state {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        if let Some(m) = &mut self._state {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._state {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(294);
        self._state = Default::default();
        self._type = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._state);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_inline_message!(bfr, self._state);
        bfr.put_u8(self._type);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = deserialize_inline_as::<EstimatedState>(bfr).ok();
        self._type = bfr.get_u8();
        Ok(())
    }
}
