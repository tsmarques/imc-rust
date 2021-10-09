use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

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

impl Message for PlanDBState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PlanDBState {
            header: Header::new(557),

            _plan_count: Default::default(),
            _plan_size: Default::default(),
            _change_time: Default::default(),
            _change_sid: Default::default(),
            _change_sname: Default::default(),
            _md5: Default::default(),
            _plans_info: vec![],
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PlanDBState {
            header: hdr,

            _plan_count: Default::default(),
            _plan_size: Default::default(),
            _change_time: Default::default(),
            _change_sid: Default::default(),
            _change_sname: Default::default(),
            _md5: Default::default(),
            _plans_info: vec![],
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        557
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        557
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._plan_count = Default::default();

        self._plan_size = Default::default();

        self._change_time = Default::default();

        self._change_sid = Default::default();

        self._change_sname = Default::default();

        self._md5 = Default::default();

        self._plans_info = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._change_sname.len() + 2;

        dyn_size += self._md5.len() + 2;

        message_list_serialization_size!(dyn_size, self._plans_info);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._plan_count);
        bfr.put_u32_le(self._plan_size);
        bfr.put_f64_le(self._change_time);
        bfr.put_u16_le(self._change_sid);
        serialize_bytes!(bfr, self._change_sname.as_bytes());
        serialize_bytes!(bfr, self._md5.as_slice());
        serialize_message_list!(bfr, self._plans_info);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._plan_count = bfr.get_u16_le();

        self._plan_size = bfr.get_u32_le();

        self._change_time = bfr.get_f64_le();

        self._change_sid = bfr.get_u16_le();

        deserialize_string!(bfr, self._change_sname);

        deserialize_bytes!(bfr, self._md5);

        for m in self._plans_info.iter_mut() {
            m.deserialize_fields(bfr);
        }
    }
}
