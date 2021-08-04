use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

use crate::PlanDBInformation::PlanDBInformation;

/// Characterizes the state of the entire plan database.
#[derive(Default)]
pub struct PlanDBState {
    /// IMC Header
    pub header: Header,

    /// Number of stored plans.
    pub _plan_count: u16,

    /// Size of all plans.The value equals the sum of the IMC payload
    /// sizes for 'PlanSpecification' stored in the DB.
    pub _plan_size: u32,

    /// Time of last change (Epoch time).
    pub _change_time: f64,

    /// IMC address for source of last DB change.
    pub _change_sid: u16,

    /// IMC node name for source of last DB change.
    pub _change_sname: String,

    /// MD5 database verification code. The MD5 hash sum is computed
    /// over the stream formed by the MD5 of all plans, ordered by
    /// plan id, in compliance with RFC 1321.
    pub _md5: Vec<u8>,

    /// Individual information for plans.
    pub _plans_info: MessageList<PlanDBInformation>,
}

impl PlanDBState {
    pub fn new() -> PlanDBState {
        let mut msg = PlanDBState {
            header: Header::new(557),

            _plan_count: Default::default(),
            _plan_size: Default::default(),
            _change_time: Default::default(),
            _change_sid: Default::default(),
            _change_sname: Default::default(),
            _md5: Default::default(),
            _plans_info: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PlanDBState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        557
    }

    fn clear(&mut self) {
        self.header.clear();

        self._plan_count = Default::default();

        self._plan_size = Default::default();

        self._change_time = Default::default();

        self._change_sid = Default::default();

        self._change_sname = Default::default();

        self._md5 = Default::default();

        for msg in self._plans_info.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        16
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._change_sname.len() + 2;

        dyn_size += self._md5.len() + 2;

        for msg in self._plans_info.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._plan_count);
        bfr.put_u32_le(self._plan_size);
        bfr.put_f64_le(self._change_time);
        bfr.put_u16_le(self._change_sid);
        serialize_bytes!(bfr, self._change_sname.as_bytes());
        serialize_bytes!(bfr, self._md5.as_slice());
        for msg in self._plans_info.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
    }
}
