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

use crate::Header;
use crate::Message;

/// Restricted Communication Means
#[allow(non_camel_case_types)]
pub mod RestrictedCommunicationMeansBits {
    /// Satellite
    pub const MEAN_SATELLITE: u32 = 0x01;
    /// Acoustic
    pub const MEAN_ACOUSTIC: u32 = 0x02;
    /// WiFi
    pub const MEAN_WIFI: u32 = 0x04;
    /// GSM
    pub const MEAN_GSM: u32 = 0x08;
}

/// This message is used to restrict the vehicle from using some communication means.
#[derive(Default, Clone)]
pub struct CommRestriction {
    /// Message Header
    pub _header: Header,
    /// The restricted communication means.
    pub _restriction: u8,
    /// Textual description for why this restriction is needed.
    pub _reason: String,
}

impl Message for CommRestriction {
    fn new() -> CommRestriction {
        CommRestriction {
            _header: Header::new(2010),
            _restriction: Default::default(),
            _reason: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        2010
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2010
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

    fn clear(&mut self) {
        self._header = Header::new(2010);
        self._restriction = Default::default();
        self._reason = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._reason.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._restriction);
        serialize_bytes!(bfr, self._reason.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._restriction = bfr.get_u8();
        deserialize_string!(bfr, self._reason);
        Ok(())
    }
}
