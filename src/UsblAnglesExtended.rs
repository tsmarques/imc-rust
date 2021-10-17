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

/// This message contains information, collected using USBL, about the
/// bearing and elevation of a target.
#[derive(Default)]
pub struct UsblAnglesExtended {
    /// Message Header.
    pub _header: Header,
    /// Target.
    pub _target: String,
    /// Local Bearing.
    pub _lbearing: f32,
    /// Local Elevation.
    pub _lelevation: f32,
    /// Bearing.
    pub _bearing: f32,
    /// Elevation.
    pub _elevation: f32,
    /// Roll Angle.
    pub _phi: f32,
    /// Pitch Angle.
    pub _theta: f32,
    /// Yaw Angle.
    pub _psi: f32,
    /// Accuracy.
    pub _accuracy: f32,
}

impl Message for UsblAnglesExtended {
    fn new() -> UsblAnglesExtended
    where
        Self: Sized,
    {
        let msg = UsblAnglesExtended {
            _header: Header::new(898),
            _target: Default::default(),
            _lbearing: Default::default(),
            _lelevation: Default::default(),
            _bearing: Default::default(),
            _elevation: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _accuracy: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        898
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        898
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(898);
        self._target = Default::default();
        self._lbearing = Default::default();
        self._lelevation = Default::default();
        self._bearing = Default::default();
        self._elevation = Default::default();
        self._phi = Default::default();
        self._theta = Default::default();
        self._psi = Default::default();
        self._accuracy = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        32
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._target.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._target.as_bytes());
        bfr.put_f32_le(self._lbearing);
        bfr.put_f32_le(self._lelevation);
        bfr.put_f32_le(self._bearing);
        bfr.put_f32_le(self._elevation);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
        bfr.put_f32_le(self._accuracy);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._target);
        self._lbearing = bfr.get_f32_le();
        self._lelevation = bfr.get_f32_le();
        self._bearing = bfr.get_f32_le();
        self._elevation = bfr.get_f32_le();
        self._phi = bfr.get_f32_le();
        self._theta = bfr.get_f32_le();
        self._psi = bfr.get_f32_le();
        self._accuracy = bfr.get_f32_le();
        Ok(())
    }
}
