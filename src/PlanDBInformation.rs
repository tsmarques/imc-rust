use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[derive(Default)]
pub struct PlanDBInformation {
    /// IMC Header
    pub header: Header,

    /// Plan identifier.
    pub _plan_id: String,

    /// Plan size. The value equals the IMC message payload of the
    /// associated 'PlanSpecification' message in bytes.
    pub _plan_size: u16,

    /// Time of last change to the plan (Epoch time).
    pub _change_time: f64,

    /// IMC address for source of last change to the plan.
    pub _change_sid: u16,

    /// IMC node name for source of last change to the plan.
    pub _change_sname: String,

    /// MD5 plan verification code. The value is calculated over the
    /// message payload of the 'PlanSpecification', in compliance with
    /// RFC 1321.
    pub _md5: Vec<u8>,
}

impl Message for PlanDBInformation {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PlanDBInformation {
            header: Header::new(558),

            _plan_id: Default::default(),
            _plan_size: Default::default(),
            _change_time: Default::default(),
            _change_sid: Default::default(),
            _change_sname: Default::default(),
            _md5: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PlanDBInformation {
            header: hdr,

            _plan_id: Default::default(),
            _plan_size: Default::default(),
            _change_time: Default::default(),
            _change_sid: Default::default(),
            _change_sname: Default::default(),
            _md5: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        558
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        558
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._plan_id = Default::default();
        self._plan_size = Default::default();
        self._change_time = Default::default();
        self._change_sid = Default::default();
        self._change_sname = Default::default();
        self._md5 = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        12
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._plan_id.len() + 2;

        dyn_size += self._change_sname.len() + 2;

        dyn_size += self._md5.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_u16_le(self._plan_size);
        bfr.put_f64_le(self._change_time);
        bfr.put_u16_le(self._change_sid);
        serialize_bytes!(bfr, self._change_sname.as_bytes());
        serialize_bytes!(bfr, self._md5.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._plan_id);
        self._plan_size = bfr.get_u16_le();
        self._change_time = bfr.get_f64_le();
        self._change_sid = bfr.get_u16_le();
        deserialize_string!(bfr, self._change_sname);
        deserialize_bytes!(bfr, self._md5);

        Ok(())
    }
}
