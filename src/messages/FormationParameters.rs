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
use crate::Header;
use crate::Message;
use crate::MessageList;
use crate::VehicleFormationParticipant;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Formation Reference Frame
#[allow(non_camel_case_types)]
pub enum FormationReferenceFrameEnum {
    /// Earth Fixed
    OP_EARTH_FIXED = 0,
    /// Path Fixed
    OP_PATH_FIXED = 1,
    /// Path Curved
    OP_PATH_CURVED = 2,
}

/// A "Formation" is defined by the relative positions of the vehicles
/// inside the formation, and the reference frame where this positions are defined.
/// The formation reference frame may be:
/// - Earth Fixed: Where the vehicles relative position do not depend on the followed path.
/// This results in all UAVs following the same path with an offset relative to each other;
/// - Path Fixed:  Where the vehicles relative position depends on the followed path,
/// changing the inter-vehicle offset direction with the path direction.
/// - Path Curved:  Where the vehicles relative position depends on the followed path,
/// changing the inter-vehicle offset direction with the path direction and direction
/// change rate.
/// An offset in the xx axis results in a distance over the curved path line.
/// An offset in the yy axis results in an offset of the vehicle path line relative to the
/// formation center path line.
#[derive(Default, Clone)]
pub struct FormationParameters {
    /// Message Header
    pub _header: Header,
    /// Name of the formation configuration.
    pub _formation_name: String,
    /// Formation reference frame
    pub _reference_frame: u8,
    /// List of formation participants.
    pub _participants: MessageList<VehicleFormationParticipant>,
    /// Custom settings for the formation configuration.
    pub _custom: String,
}

impl Message for FormationParameters {
    fn new() -> FormationParameters {
        FormationParameters {
            _header: Header::new(476),
            _formation_name: Default::default(),
            _reference_frame: Default::default(),
            _participants: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        476
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        476
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&self) -> &Header {
        &self._header
    }

    fn get_mut_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_mut_header()._timestamp = ts;
        for m in &mut self._participants {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        for m in &mut self._participants {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        for m in &mut self._participants {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        for m in &mut self._participants {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        for m in &mut self._participants {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(476);
        self._formation_name = Default::default();
        self._reference_frame = Default::default();
        self._participants = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._formation_name.len() + 2;
        message_list_serialization_size!(dyn_size, self._participants);
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._formation_name.as_bytes());
        bfr.put_u8(self._reference_frame);
        serialize_message_list!(bfr, self._participants);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._formation_name);
        self._reference_frame = bfr.get_u8();
        self._participants = deserialize_message_list_as::<VehicleFormationParticipant>(bfr)?;
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
