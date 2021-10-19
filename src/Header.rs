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

/// The packet header contains handling information in the form of
/// supplemental fields, it is always placed at the beginning of a
/// packet.
#[derive(Default, PartialEq, Debug)]
pub struct Header {
    /// Synchronization Number
    pub _sync: u16,
    /// Message Identification Number
    pub _mgid: u16,
    /// Message size
    pub _size: u16,
    /// Time stamp
    pub _timestamp: f64,
    /// Source Address
    pub _src: u16,
    /// Source Entity
    pub _src_ent: u8,
    /// Destination Address
    pub _dst: u16,
    /// Destination Entity
    pub _dst_ent: u8,
}

impl Header {
    pub fn new(msg_id: u16) -> Header {
        let mut header = Header {
            _sync: 0xFE54_u16,
            _mgid: Default::default(),
            _size: Default::default(),
            _timestamp: Default::default(),
            _src: Default::default(),
            _src_ent: Default::default(),
            _dst: Default::default(),
            _dst_ent: Default::default(),
        };

        header._mgid = msg_id;
        header
    }

    pub fn clear(&mut self) {
        self._sync = 0xFE54_u16;
        self._mgid = Default::default();
        self._size = Default::default();
        self._timestamp = Default::default();
        self._src = Default::default();
        self._src_ent = Default::default();
        self._dst = Default::default();
        self._dst_ent = Default::default()
    }
    pub fn serialize(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._sync);
        bfr.put_u16_le(self._mgid);
        bfr.put_u16_le(self._size);
        bfr.put_f64_le(self._timestamp);
        bfr.put_u16_le(self._src);
        bfr.put_u8(self._src_ent);
        bfr.put_u16_le(self._dst);
        bfr.put_u8(self._dst_ent);
    }
}
