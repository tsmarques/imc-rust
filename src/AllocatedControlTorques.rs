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
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Control torques allocated to the actuators.
#[derive(Default, Clone)]
pub struct AllocatedControlTorques {
    /// Message Header
    pub _header: Header,
    /// Torque K about the vehicle's x axis.
    pub _k: f64,
    /// Torque M about the vehicle's y axis.
    pub _m: f64,
    /// Torque N about the vehicle's z axis.
    pub _n: f64,
}

impl Message for AllocatedControlTorques {
    fn new() -> AllocatedControlTorques {
        let msg = AllocatedControlTorques {
            _header: Header::new(411),
            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        411
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        411
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
        self._header = Header::new(411);
        self._k = Default::default();
        self._m = Default::default();
        self._n = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        24
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._k);
        bfr.put_f64_le(self._m);
        bfr.put_f64_le(self._n);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._k = bfr.get_f64_le();
        self._m = bfr.get_f64_le();
        self._n = bfr.get_f64_le();
        Ok(())
    }
}
