//###########################################################################
// Copyright 2021 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Report of navigation data.
/// This is constituted by data which is not
/// part of the vehicle estimated state but
/// that the user may refer for more information.
#[derive(Default, Clone)]
pub struct NavigationData {
    /// Message Header
    pub _header: Header,
    /// The psi Euler angle bias from the vehicle's sensed attitude.
    pub _bias_psi: f32,
    /// The angular velocity over body-fixed zz axis bias from sensor.
    pub _bias_r: f32,
    /// Course over ground given by NED ground velocity vectors.
    pub _cog: f32,
    /// Continuous psi Euler angle (without normalizations).
    pub _cyaw: f32,
    /// GPS rejection filter level.
    pub _lbl_rej_level: f32,
    /// LBL rejection filter level.
    pub _gps_rej_level: f32,
    /// Custom variable.
    pub _custom_x: f32,
    /// Custom variable.
    pub _custom_y: f32,
    /// Custom variable.
    pub _custom_z: f32,
}

impl Message for NavigationData {
    fn new() -> NavigationData {
        

        NavigationData {
            _header: Header::new(355),
            _bias_psi: Default::default(),
            _bias_r: Default::default(),
            _cog: Default::default(),
            _cyaw: Default::default(),
            _lbl_rej_level: Default::default(),
            _gps_rej_level: Default::default(),
            _custom_x: Default::default(),
            _custom_y: Default::default(),
            _custom_z: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        355
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        355
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

    fn clear(&mut self) {
        self._header = Header::new(355);
        self._bias_psi = Default::default();
        self._bias_r = Default::default();
        self._cog = Default::default();
        self._cyaw = Default::default();
        self._lbl_rej_level = Default::default();
        self._gps_rej_level = Default::default();
        self._custom_x = Default::default();
        self._custom_y = Default::default();
        self._custom_z = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        36
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._bias_psi);
        bfr.put_f32_le(self._bias_r);
        bfr.put_f32_le(self._cog);
        bfr.put_f32_le(self._cyaw);
        bfr.put_f32_le(self._lbl_rej_level);
        bfr.put_f32_le(self._gps_rej_level);
        bfr.put_f32_le(self._custom_x);
        bfr.put_f32_le(self._custom_y);
        bfr.put_f32_le(self._custom_z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._bias_psi = bfr.get_f32_le();
        self._bias_r = bfr.get_f32_le();
        self._cog = bfr.get_f32_le();
        self._cyaw = bfr.get_f32_le();
        self._lbl_rej_level = bfr.get_f32_le();
        self._gps_rej_level = bfr.get_f32_le();
        self._custom_x = bfr.get_f32_le();
        self._custom_y = bfr.get_f32_le();
        self._custom_z = bfr.get_f32_le();
        Ok(())
    }
}
