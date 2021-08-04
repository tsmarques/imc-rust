use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum OperationEnum {
    // Request Start of Reports
    OP_REQUEST_START = 0,
    // Report Started
    OP_STARTED = 1,
    // Request Stop of Reports
    OP_REQUEST_STOP = 2,
    // Report Stopped
    OP_STOPPED = 3,
    // Request Single Reports
    OP_REQUEST_REPORT = 4,
    // Single Report Sent
    OP_REPORT_SENT = 5,
}

impl OperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            OP_REQUEST_START => 0,
            OP_STARTED => 1,
            OP_REQUEST_STOP => 2,
            OP_STOPPED => 3,
            OP_REQUEST_REPORT => 4,
            OP_REPORT_SENT => 5,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum CommunicationInterfaceEnum {
    // Acoustic
    CI_ACOUSTIC = 0x01,
    // Satellite
    CI_SATELLITE = 0x02,
    // GSM
    CI_GSM = 0x04,
    // Mobile
    CI_MOBILE = 0x08,
}

impl CommunicationInterfaceEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            CI_ACOUSTIC => 0x01,
            CI_SATELLITE => 0x02,
            CI_GSM => 0x04,
            CI_MOBILE => 0x08,
        }
    }
}

/// Use Global System for Mobile Communications
#[derive(Default)]
pub struct ReportControl {
    /// IMC Header
    pub header: Header,

    /// The destination system will reply a single report request with
    /// this operation.If applicable, the destination address is defined
    /// in field 'dst'.
    pub _op: u8,

    /// Use mobile networks
    pub _comm_interface: u8,

    /// Desired periodicity for scheduled reports.
    pub _period: u16,

    /// Destination Address to be filled where applicable. It should be
    /// interpreted differently depending on communication interface.
    pub _sys_dst: String,
}

impl ReportControl {
    pub fn new() -> ReportControl {
        let mut msg = ReportControl {
            header: Header::new(513),

            _op: Default::default(),
            _comm_interface: Default::default(),
            _period: Default::default(),
            _sys_dst: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ReportControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        513
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._comm_interface = Default::default();

        self._period = Default::default();

        self._sys_dst = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        4
    }

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
}
