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
// Author: Ricardo Martins                                                  #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Author: Tiago Sá Marques <tmarques@oceanscan-mst.com>

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Report of navigation data.
/// This is constituted by data which is not
/// part of the vehicle estimated state but
/// that the user may refer for more information.
#[derive(Default)]
pub struct NavigationData {
    /// Message Header.
    pub _header: Header,
    /// Yaw Bias.
    pub _bias_psi: f32,
    /// Gyro. Yaw Rate Bias.
    pub _bias_r: f32,
    /// Course Over Ground.
    pub _cog: f32,
    /// Continuous Yaw.
    pub _cyaw: f32,
    /// GPS Rejection Filter Level.
    pub _lbl_rej_level: f32,
    /// LBL Rejection Filter Level.
    pub _gps_rej_level: f32,
    /// Variance - Custom Variable X.
    pub _custom_x: f32,
    /// Variance - Custom Variable Y.
    pub _custom_y: f32,
    /// Variance - Custom Variable Z.
    pub _custom_z: f32,
}

impl Message for NavigationData {
    fn new() -> NavigationData
    where
        Self: Sized,
    {
        let msg = NavigationData {
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
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        355
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        355
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
