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
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::RelativeState::RelativeState;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Monitoring variables for the formation state and performance.
#[derive(Default, Clone)]
pub struct FormationMonitor {
    /// Message Header
    pub _header: Header,
    /// Commanded acceleration computed by the formation controller: northward direction.
    /// On the vehicle directional reference frame.
    /// Constrained by the vehicle operational limits.
    pub _ax_cmd: f32,
    /// Commanded acceleration computed by the formation controller: eastward direction.
    /// On the vehicle directional reference frame.
    /// Constrained by the vehicle operational limits.
    pub _ay_cmd: f32,
    /// Commanded acceleration computed by the formation controller: downward direction.
    /// On the vehicle directional reference frame.
    /// Constrained by the vehicle operational limits.
    pub _az_cmd: f32,
    /// Desired acceleration computed by the formation controller: northward direction.
    /// On the fixed reference frame.
    pub _ax_des: f32,
    /// Desired acceleration computed by the formation controller: eastward direction.
    /// On the fixed reference frame.
    pub _ay_des: f32,
    /// Desired acceleration computed by the formation controller: downward direction.
    /// On the fixed reference frame.
    pub _az_des: f32,
    /// Components of the vehicle desired acceleration.
    /// Overall formation combined virtual error: northward direction.
    /// On the fixed reference frame.
    pub _virt_err_x: f32,
    /// Components of the vehicle desired acceleration.
    /// Overall formation combined virtual error: eastward direction.
    /// On the fixed reference frame.
    pub _virt_err_y: f32,
    /// Components of the vehicle desired acceleration.
    /// Overall formation combined virtual error: downward direction.
    /// On the fixed reference frame.
    pub _virt_err_z: f32,
    /// Components of the vehicle desired acceleration.
    /// Overall formation combined sliding surface feedback: northward direction.
    /// On the fixed reference frame.
    pub _surf_fdbk_x: f32,
    /// Components of the vehicle desired acceleration.
    /// Overall formation combined sliding surface feedback: eastward direction.
    /// On the fixed reference frame.
    pub _surf_fdbk_y: f32,
    /// Components of the vehicle desired acceleration.
    /// Overall formation combined sliding surface feedback: downward direction.
    /// On the fixed reference frame.
    pub _surf_fdbk_z: f32,
    /// Components of the vehicle desired acceleration.
    /// Dynamics uncertainty compensation: northward direction.
    pub _surf_unkn_x: f32,
    /// Components of the vehicle desired acceleration.
    /// Dynamics uncertainty compensation: eastward direction.
    pub _surf_unkn_y: f32,
    /// Components of the vehicle desired acceleration.
    /// Dynamics uncertainty compensation: downward direction.
    pub _surf_unkn_z: f32,
    /// Combined deviation from convergence (sliding surface): North component.
    pub _ss_x: f32,
    /// Combined deviation from convergence (sliding surface): East component.
    pub _ss_y: f32,
    /// Combined deviation from convergence (sliding surface): Down component.
    pub _ss_z: f32,
    /// List of RelativeState messages, encoding the inter-vehicle formation state.
    pub _rel_state: MessageList<RelativeState>,
}

impl Message for FormationMonitor {
    fn new() -> FormationMonitor {
        

        FormationMonitor {
            _header: Header::new(481),
            _ax_cmd: Default::default(),
            _ay_cmd: Default::default(),
            _az_cmd: Default::default(),
            _ax_des: Default::default(),
            _ay_des: Default::default(),
            _az_des: Default::default(),
            _virt_err_x: Default::default(),
            _virt_err_y: Default::default(),
            _virt_err_z: Default::default(),
            _surf_fdbk_x: Default::default(),
            _surf_fdbk_y: Default::default(),
            _surf_fdbk_z: Default::default(),
            _surf_unkn_x: Default::default(),
            _surf_unkn_y: Default::default(),
            _surf_unkn_z: Default::default(),
            _ss_x: Default::default(),
            _ss_y: Default::default(),
            _ss_z: Default::default(),
            _rel_state: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        481
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        481
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._rel_state {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._rel_state {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._rel_state {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._rel_state {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._rel_state {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(481);
        self._ax_cmd = Default::default();
        self._ay_cmd = Default::default();
        self._az_cmd = Default::default();
        self._ax_des = Default::default();
        self._ay_des = Default::default();
        self._az_des = Default::default();
        self._virt_err_x = Default::default();
        self._virt_err_y = Default::default();
        self._virt_err_z = Default::default();
        self._surf_fdbk_x = Default::default();
        self._surf_fdbk_y = Default::default();
        self._surf_fdbk_z = Default::default();
        self._surf_unkn_x = Default::default();
        self._surf_unkn_y = Default::default();
        self._surf_unkn_z = Default::default();
        self._ss_x = Default::default();
        self._ss_y = Default::default();
        self._ss_z = Default::default();
        self._rel_state = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        72
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._rel_state);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._ax_cmd);
        bfr.put_f32_le(self._ay_cmd);
        bfr.put_f32_le(self._az_cmd);
        bfr.put_f32_le(self._ax_des);
        bfr.put_f32_le(self._ay_des);
        bfr.put_f32_le(self._az_des);
        bfr.put_f32_le(self._virt_err_x);
        bfr.put_f32_le(self._virt_err_y);
        bfr.put_f32_le(self._virt_err_z);
        bfr.put_f32_le(self._surf_fdbk_x);
        bfr.put_f32_le(self._surf_fdbk_y);
        bfr.put_f32_le(self._surf_fdbk_z);
        bfr.put_f32_le(self._surf_unkn_x);
        bfr.put_f32_le(self._surf_unkn_y);
        bfr.put_f32_le(self._surf_unkn_z);
        bfr.put_f32_le(self._ss_x);
        bfr.put_f32_le(self._ss_y);
        bfr.put_f32_le(self._ss_z);
        serialize_message_list!(bfr, self._rel_state);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._ax_cmd = bfr.get_f32_le();
        self._ay_cmd = bfr.get_f32_le();
        self._az_cmd = bfr.get_f32_le();
        self._ax_des = bfr.get_f32_le();
        self._ay_des = bfr.get_f32_le();
        self._az_des = bfr.get_f32_le();
        self._virt_err_x = bfr.get_f32_le();
        self._virt_err_y = bfr.get_f32_le();
        self._virt_err_z = bfr.get_f32_le();
        self._surf_fdbk_x = bfr.get_f32_le();
        self._surf_fdbk_y = bfr.get_f32_le();
        self._surf_fdbk_z = bfr.get_f32_le();
        self._surf_unkn_x = bfr.get_f32_le();
        self._surf_unkn_y = bfr.get_f32_le();
        self._surf_unkn_z = bfr.get_f32_le();
        self._ss_x = bfr.get_f32_le();
        self._ss_y = bfr.get_f32_le();
        self._ss_z = bfr.get_f32_le();
        self._rel_state = deserialize_message_list_as::<RelativeState>(bfr)?;
        Ok(())
    }
}
