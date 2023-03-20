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

use crate::Header;
use crate::Message;

/// Inter-vehicle formation state.
#[derive(Default, Clone)]
pub struct RelativeState {
    /// Message Header
    pub _header: Header,
    /// The identifier of the vehicle whose relative state is being reported.
    pub _s_id: String,
    /// Distance between vehicles.
    pub _dist: f32,
    /// Relative position error norm.
    pub _err: f32,
    /// Weight in the computation of the desired acceleration.
    pub _ctrl_imp: f32,
    /// Inter-vehicle direction vector: North component.
    pub _rel_dir_x: f32,
    /// Inter-vehicle direction vector: East component.
    pub _rel_dir_y: f32,
    /// Inter-vehicle direction vector: Down component.
    pub _rel_dir_z: f32,
    /// Relative position error: North component.
    pub _err_x: f32,
    /// Relative position error: East component.
    pub _err_y: f32,
    /// Relative position error: Down component.
    pub _err_z: f32,
    /// Relative position error: X component on the inter-vehicle reference frame.
    pub _rf_err_x: f32,
    /// Relative position error: Y component on the inter-vehicle reference frame.
    pub _rf_err_y: f32,
    /// Relative position error: Z component on the inter-vehicle reference frame.
    pub _rf_err_z: f32,
    /// Relative veloctity error: X component in the inter-vehicle reference frame.
    pub _rf_err_vx: f32,
    /// Relative velocity error: Y component on the inter-vehicle reference frame.
    pub _rf_err_vy: f32,
    /// Relative velocity error: Z component on the inter-vehicle reference frame.
    pub _rf_err_vz: f32,
    /// Deviation from convergence (sliding surface): X component on the inter-vehicle reference frame.
    pub _ss_x: f32,
    /// Deviation from convergence (sliding surface): Y component on the inter-vehicle reference frame.
    pub _ss_y: f32,
    /// Deviation from convergence (sliding surface): Z component on the inter-vehicle reference frame.
    pub _ss_z: f32,
    /// Components of the vehicle desired acceleration.
    /// Relative virtual error: northward direction.
    pub _virt_err_x: f32,
    /// Components of the vehicle desired acceleration.
    /// Relative virtual error: eastward direction.
    pub _virt_err_y: f32,
    /// Components of the vehicle desired acceleration.
    /// Relative virtual error: downward direction.
    pub _virt_err_z: f32,
}

impl Message for RelativeState {
    fn new() -> RelativeState {
        RelativeState {
            _header: Header::new(482),
            _s_id: Default::default(),
            _dist: Default::default(),
            _err: Default::default(),
            _ctrl_imp: Default::default(),
            _rel_dir_x: Default::default(),
            _rel_dir_y: Default::default(),
            _rel_dir_z: Default::default(),
            _err_x: Default::default(),
            _err_y: Default::default(),
            _err_z: Default::default(),
            _rf_err_x: Default::default(),
            _rf_err_y: Default::default(),
            _rf_err_z: Default::default(),
            _rf_err_vx: Default::default(),
            _rf_err_vy: Default::default(),
            _rf_err_vz: Default::default(),
            _ss_x: Default::default(),
            _ss_y: Default::default(),
            _ss_z: Default::default(),
            _virt_err_x: Default::default(),
            _virt_err_y: Default::default(),
            _virt_err_z: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        482
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        482
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
        self._header = Header::new(482);
        self._s_id = Default::default();
        self._dist = Default::default();
        self._err = Default::default();
        self._ctrl_imp = Default::default();
        self._rel_dir_x = Default::default();
        self._rel_dir_y = Default::default();
        self._rel_dir_z = Default::default();
        self._err_x = Default::default();
        self._err_y = Default::default();
        self._err_z = Default::default();
        self._rf_err_x = Default::default();
        self._rf_err_y = Default::default();
        self._rf_err_z = Default::default();
        self._rf_err_vx = Default::default();
        self._rf_err_vy = Default::default();
        self._rf_err_vz = Default::default();
        self._ss_x = Default::default();
        self._ss_y = Default::default();
        self._ss_z = Default::default();
        self._virt_err_x = Default::default();
        self._virt_err_y = Default::default();
        self._virt_err_z = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        84
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._s_id.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._s_id.as_bytes());
        bfr.put_f32_le(self._dist);
        bfr.put_f32_le(self._err);
        bfr.put_f32_le(self._ctrl_imp);
        bfr.put_f32_le(self._rel_dir_x);
        bfr.put_f32_le(self._rel_dir_y);
        bfr.put_f32_le(self._rel_dir_z);
        bfr.put_f32_le(self._err_x);
        bfr.put_f32_le(self._err_y);
        bfr.put_f32_le(self._err_z);
        bfr.put_f32_le(self._rf_err_x);
        bfr.put_f32_le(self._rf_err_y);
        bfr.put_f32_le(self._rf_err_z);
        bfr.put_f32_le(self._rf_err_vx);
        bfr.put_f32_le(self._rf_err_vy);
        bfr.put_f32_le(self._rf_err_vz);
        bfr.put_f32_le(self._ss_x);
        bfr.put_f32_le(self._ss_y);
        bfr.put_f32_le(self._ss_z);
        bfr.put_f32_le(self._virt_err_x);
        bfr.put_f32_le(self._virt_err_y);
        bfr.put_f32_le(self._virt_err_z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._s_id);
        self._dist = bfr.get_f32_le();
        self._err = bfr.get_f32_le();
        self._ctrl_imp = bfr.get_f32_le();
        self._rel_dir_x = bfr.get_f32_le();
        self._rel_dir_y = bfr.get_f32_le();
        self._rel_dir_z = bfr.get_f32_le();
        self._err_x = bfr.get_f32_le();
        self._err_y = bfr.get_f32_le();
        self._err_z = bfr.get_f32_le();
        self._rf_err_x = bfr.get_f32_le();
        self._rf_err_y = bfr.get_f32_le();
        self._rf_err_z = bfr.get_f32_le();
        self._rf_err_vx = bfr.get_f32_le();
        self._rf_err_vy = bfr.get_f32_le();
        self._rf_err_vz = bfr.get_f32_le();
        self._ss_x = bfr.get_f32_le();
        self._ss_y = bfr.get_f32_le();
        self._ss_z = bfr.get_f32_le();
        self._virt_err_x = bfr.get_f32_le();
        self._virt_err_y = bfr.get_f32_le();
        self._virt_err_z = bfr.get_f32_le();
        Ok(())
    }
}
