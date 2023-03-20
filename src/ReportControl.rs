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

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Request Start of Reports
    OP_REQUEST_START = 0,
    /// Report Started
    OP_STARTED = 1,
    /// Request Stop of Reports
    OP_REQUEST_STOP = 2,
    /// Report Stopped
    OP_STOPPED = 3,
    /// Request Single Reports
    OP_REQUEST_REPORT = 4,
    /// Single Report Sent
    OP_REPORT_SENT = 5,
}

/// Communication Interface
#[allow(non_camel_case_types)]
pub mod CommunicationInterfaceBits {
    /// Acoustic
    pub const CI_ACOUSTIC: u32 = 0x01;
    /// Satellite
    pub const CI_SATELLITE: u32 = 0x02;
    /// GSM
    pub const CI_GSM: u32 = 0x04;
    /// Mobile
    pub const CI_MOBILE: u32 = 0x08;
    /// Radio
    pub const CI_RADIO: u32 = 0x10;
}

/// This message is sent to trigger reports to a destination system.
#[derive(Default, Clone)]
pub struct ReportControl {
    /// Message Header
    pub _header: Header,
    /// Operation to perform.
    pub _op: u8,
    /// Communication interface to be used for reports.
    pub _comm_interface: u8,
    /// Desired periodicity for scheduled reports.
    pub _period: u16,
    /// Destination Address to be filled where applicable. It should be
    /// interpreted differently depending on communication interface.
    pub _sys_dst: String,
}

impl Message for ReportControl {
    fn new() -> ReportControl {
        ReportControl {
            _header: Header::new(513),
            _op: Default::default(),
            _comm_interface: Default::default(),
            _period: Default::default(),
            _sys_dst: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        513
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        513
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
        self._header = Header::new(513);
        self._op = Default::default();
        self._comm_interface = Default::default();
        self._period = Default::default();
        self._sys_dst = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._sys_dst.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        bfr.put_u8(self._comm_interface);
        bfr.put_u16_le(self._period);
        serialize_bytes!(bfr, self._sys_dst.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._comm_interface = bfr.get_u8();
        self._period = bfr.get_u16_le();
        deserialize_string!(bfr, self._sys_dst);
        Ok(())
    }
}
