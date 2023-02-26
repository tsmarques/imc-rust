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

use crate::Header::Header;
use crate::Message::*;

/// Model
#[allow(non_camel_case_types)]
pub enum ModelEnum {
    /// unknown
    CIQ_UNKNOWN = 0x00,
    /// 3DR
    CIQ_M3DR = 0x01,
    /// RDFXXXxPtP
    CIQ_RDFXXXXPTP = 0x02,
}

/// Type
#[allow(non_camel_case_types)]
pub mod TypeBits {
    /// Query Systems
    pub const CIQ_QUERY: u32 = 0x01;
    /// Reply
    pub const CIQ_REPLY: u32 = 0x02;
}

/// Communication Interface
#[allow(non_camel_case_types)]
pub mod CommunicationInterfaceBits {
    /// Acoustic
    pub const CIQ_ACOUSTIC: u32 = 0x01;
    /// Satellite
    pub const CIQ_SATELLITE: u32 = 0x02;
    /// GSM
    pub const CIQ_GSM: u32 = 0x04;
    /// Mobile
    pub const CIQ_MOBILE: u32 = 0x08;
    /// Radio
    pub const CIQ_RADIO: u32 = 0x10;
}

/// Presence of Communication Interfaces query.
#[derive(Default, Clone)]
pub struct CommSystemsQuery {
    /// Message Header
    pub _header: Header,
    pub _type: u8,
    /// Communication interface to be used for reports.
    pub _comm_interface: u16,
    pub _model: u16,
    /// Comma separated list of known Radio system names.
    pub _list: String,
}

impl Message for CommSystemsQuery {
    fn new() -> CommSystemsQuery {
        

        CommSystemsQuery {
            _header: Header::new(189),
            _type: Default::default(),
            _comm_interface: Default::default(),
            _model: Default::default(),
            _list: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        189
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        189
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

    fn clear(&mut self) {
        self._header = Header::new(189);
        self._type = Default::default();
        self._comm_interface = Default::default();
        self._model = Default::default();
        self._list = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._list.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u16_le(self._comm_interface);
        bfr.put_u16_le(self._model);
        serialize_bytes!(bfr, self._list.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._comm_interface = bfr.get_u16_le();
        self._model = bfr.get_u16_le();
        deserialize_string!(bfr, self._list);
        Ok(())
    }
}
