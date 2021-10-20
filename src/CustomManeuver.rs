//###########################################################################
// Copyright 2021 OceanScan - Marine Systems & Technology, Lda.             #
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

use crate::Header::Header;
use crate::Message::*;

/// The Custom Maneuver message may be used as specification of a
/// very specific maneuver not covered by the IMC scope. The
/// settings of the maneuver are just its id, timeout and other
/// settings encoded as a tuple list.
#[derive(Default)]
pub struct CustomManeuver {
    /// Message Header
    pub _header: Header,
    /// The amount of time the maneuver is allowed to run. If the
    /// maneuver is not completed in the amount of time specified an
    /// error will be generated.
    pub _timeout: u16,
    /// The maneuver name, used as key by an implementation to bind
    /// the maneuver to the corresponding controller.
    pub _name: String,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for CustomManeuver {
    fn new() -> CustomManeuver {
        let msg = CustomManeuver {
            _header: Header::new(465),
            _timeout: Default::default(),
            _name: Default::default(),
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        465
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        465
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(465);
        self._timeout = Default::default();
        self._name = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._name.len() + 2;
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._timeout);
        serialize_bytes!(bfr, self._name.as_bytes());
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u16_le();
        deserialize_string!(bfr, self._name);
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
