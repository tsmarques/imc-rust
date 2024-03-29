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

/// ServiceType
#[allow(non_camel_case_types)]
pub mod ServiceTypeBits {
    /// External
    pub const SRV_TYPE_EXTERNAL: u32 = 0x01;
    /// Local
    pub const SRV_TYPE_LOCAL: u32 = 0x02;
}

/// Announcement about the existence of a service.
#[derive(Default, Clone)]
pub struct AnnounceService {
    /// Message Header
    pub _header: Header,
    /// Semicolon separated list of URLs (see :ref:`Announce`).
    pub _service: String,
    /// Informs about the availability of the service on internal and
    /// external networks.
    pub _service_type: u8,
}

impl Message for AnnounceService {
    fn new() -> AnnounceService {
        AnnounceService {
            _header: Header::new(152),
            _service: Default::default(),
            _service_type: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        152
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        152
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
        self._header = Header::new(152);
        self._service = Default::default();
        self._service_type = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._service.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._service.as_bytes());
        bfr.put_u8(self._service_type);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._service);
        self._service_type = bfr.get_u8();
        Ok(())
    }
}
