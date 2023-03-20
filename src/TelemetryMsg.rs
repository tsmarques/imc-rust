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

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Tx
    TM_TX = 0x01,
    /// Rx
    TM_RX = 0x02,
    /// TxStatus
    TM_TXSTATUS = 0x03,
}

/// Code
#[allow(non_camel_case_types)]
pub enum CodeEnum {
    /// Code unknown
    TM_CODE_UNK = 0x00,
    /// Code Report
    TM_CODE_REPORT = 0x01,
    /// Code IMC
    TM_CODE_IMC = 0x02,
    /// Code raw
    TM_CODE_RAW = 0x03,
}

/// Status
#[allow(non_camel_case_types)]
pub enum StatusEnum {
    /// Does not apply
    TM_NONE = 0x00,
    /// Successfull transmission
    TM_DONE = 1,
    /// Error while trying to transmit message
    TM_FAILED = 2,
    /// Message has been queued for transmission
    TM_QUEUED = 3,
    /// Message is currently being transmitted
    TM_TRANSMIT = 4,
    /// Message's TTL has expired. Transmition cancelled
    TM_EXPIRED = 5,
    /// No more messages to be transmitted or received
    TM_EMPTY = 6,
    /// Invalid address
    TM_INV_ADDR = 7,
    /// Invalid transmission size
    TM_INV_SIZE = 8,
}

/// Acknowledge
#[allow(non_camel_case_types)]
pub mod AcknowledgeBits {
    /// Not acknowledge
    pub const TM_NAK: u32 = 0x00;
    /// acknowledge
    pub const TM_AK: u32 = 0x01;
}

/// Message to handle telemetry transmissions.
#[derive(Default, Clone)]
pub struct TelemetryMsg {
    /// Message Header
    pub _header: Header,
    /// Type of telemetry transmissions.
    pub _type: u8,
    /// The request identifier used to receive transmission updates.
    pub _req_id: u32,
    /// Time, in seconds, which will be considered a non-transmitted message.
    pub _ttl: u16,
    /// Type of telemetry transmissions.
    pub _code: u8,
    /// The unique identifier of this message's destination (e.g. lauv-xtreme-2, manta-0).
    pub _destination: String,
    /// The unique identifier of this message's destination (e.g. lauv-xtreme-2, manta-0).
    pub _Source: String,
    /// Type of telemetry transmissions.
    pub _acknowledge: u8,
    /// State of the transmitted message.
    pub _status: u8,
    pub _data: Vec<u8>,
}

impl Message for TelemetryMsg {
    fn new() -> TelemetryMsg {
        TelemetryMsg {
            _header: Header::new(190),
            _type: Default::default(),
            _req_id: Default::default(),
            _ttl: Default::default(),
            _code: Default::default(),
            _destination: Default::default(),
            _Source: Default::default(),
            _acknowledge: Default::default(),
            _status: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        190
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        190
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
        self._header = Header::new(190);
        self._type = Default::default();
        self._req_id = Default::default();
        self._ttl = Default::default();
        self._code = Default::default();
        self._destination = Default::default();
        self._Source = Default::default();
        self._acknowledge = Default::default();
        self._status = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        10
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._destination.len() + 2;
        dyn_size += self._Source.len() + 2;
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u32_le(self._req_id);
        bfr.put_u16_le(self._ttl);
        bfr.put_u8(self._code);
        serialize_bytes!(bfr, self._destination.as_bytes());
        serialize_bytes!(bfr, self._Source.as_bytes());
        bfr.put_u8(self._acknowledge);
        bfr.put_u8(self._status);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._req_id = bfr.get_u32_le();
        self._ttl = bfr.get_u16_le();
        self._code = bfr.get_u8();
        deserialize_string!(bfr, self._destination);
        deserialize_string!(bfr, self._Source);
        self._acknowledge = bfr.get_u8();
        self._status = bfr.get_u8();
        deserialize_bytes!(bfr, self._data);
        Ok(())
    }
}
