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

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Measurement from one specific beam at the given CellPosition.
/// Water Velocity is provided in the chosen Coordinate system.
/// Amplitude and Correlation are always in the BEAM coordinate system.
#[derive(Default, Clone)]
pub struct ADCPBeam {
    /// Message Header
    pub _header: Header,
    /// Water velocity measured in the chosen coordinate system.
    pub _vel: f32,
    /// Amplitude of returning ping for the beam.
    pub _amp: f32,
    /// Autocorrelation of returning ping for the beam.
    pub _cor: u8,
}

impl Message for ADCPBeam {
    fn new() -> ADCPBeam {
        let msg = ADCPBeam {
            _header: Header::new(1016),
            _vel: Default::default(),
            _amp: Default::default(),
            _cor: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        1016
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        1016
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(1016);
        self._vel = Default::default();
        self._amp = Default::default();
        self._cor = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        9
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._vel);
        bfr.put_f32_le(self._amp);
        bfr.put_u8(self._cor);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._vel = bfr.get_f32_le();
        self._amp = bfr.get_f32_le();
        self._cor = bfr.get_u8();
        Ok(())
    }
}
