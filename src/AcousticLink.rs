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

/// This message is used to report the perceived link quality to other
/// acoustic peers.
#[derive(Default, Clone)]
pub struct AcousticLink {
    /// Message Header
    pub _header: Header,
    /// The name of the peer on the other side of this link.
    pub _peer: String,
    /// RSSI is a signed floating point number. Higher RSSI values correspond to
    /// stronger signals.
    /// The signal strength is acceptable when measured RSSI values lie between
    /// -20 dB and -85 dB.
    pub _rssi: f32,
    /// Signal Integrity value illustrates distortion of the last received
    /// acoustic signal. It is calculated based on cross-correlation
    /// measurements.
    /// Higher *Signal Integrity Level* values correspond to less distorted
    /// signals. An acoustic link is considered weak if the *Signal Integrity
    /// Level* value is less than 100.
    pub _integrity: u16,
}

impl Message for AcousticLink {
    fn new() -> AcousticLink {
        AcousticLink {
            _header: Header::new(214),
            _peer: Default::default(),
            _rssi: Default::default(),
            _integrity: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        214
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        214
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
        self._header = Header::new(214);
        self._peer = Default::default();
        self._rssi = Default::default();
        self._integrity = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._peer.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._peer.as_bytes());
        bfr.put_f32_le(self._rssi);
        bfr.put_u16_le(self._integrity);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._peer);
        self._rssi = bfr.get_f32_le();
        self._integrity = bfr.get_u16_le();
        Ok(())
    }
}
