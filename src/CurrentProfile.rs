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
use crate::CurrentProfileCell::CurrentProfileCell;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Coordinate System
#[allow(non_camel_case_types)]
pub mod CoordinateSystemBits {
    /// xyz
    pub const UTF_XYZ: u32 = 0x01;
    /// ned
    pub const UTF_NED: u32 = 0x02;
    /// beams
    pub const UTF_BEAMS: u32 = 0x04;
}

/// Contains a profile of water velocities measured relative to the vehicle
/// velocity, represented in the specified coordinate system.
#[derive(Default, Clone)]
pub struct CurrentProfile {
    /// Message Header
    pub _header: Header,
    /// Number of ADCP beams.
    pub _nbeams: u8,
    /// Number of ADCP cells.
    pub _ncells: u8,
    /// Coordinate system of the velocity measurement.
    pub _coord_sys: u8,
    /// List of current profile measurement cells.
    pub _profile: MessageList<CurrentProfileCell>,
}

impl Message for CurrentProfile {
    fn new() -> CurrentProfile {
        

        CurrentProfile {
            _header: Header::new(1014),
            _nbeams: Default::default(),
            _ncells: Default::default(),
            _coord_sys: Default::default(),
            _profile: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        1014
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        1014
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
        for m in &mut self._profile {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._profile {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._profile {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._profile {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._profile {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(1014);
        self._nbeams = Default::default();
        self._ncells = Default::default();
        self._coord_sys = Default::default();
        self._profile = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._profile);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._nbeams);
        bfr.put_u8(self._ncells);
        bfr.put_u8(self._coord_sys);
        serialize_message_list!(bfr, self._profile);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._nbeams = bfr.get_u8();
        self._ncells = bfr.get_u8();
        self._coord_sys = bfr.get_u8();
        self._profile = deserialize_message_list_as::<CurrentProfileCell>(bfr)?;
        Ok(())
    }
}
