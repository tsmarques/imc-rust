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
use crate::LblBeacon::LblBeacon;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Operation.
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Set LBL Configuration.
    OP_SET_CFG = 0,
    /// Retrieve LBL Configuration.
    OP_GET_CFG = 1,
    /// Reply to a GET command.
    OP_CUR_CFG = 2,
}

/// Long Base Line configuration.
#[derive(Default)]
pub struct LblConfig {
    /// Message Header.
    pub _header: Header,
    /// Operation.
    pub _op: u8,
    /// Beacons.
    pub _beacons: MessageList<LblBeacon>,
}

impl Message for LblConfig {
    fn new() -> LblConfig {
        let msg = LblConfig {
            _header: Header::new(203),
            _op: Default::default(),
            _beacons: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        203
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        203
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(203);
        self._op = Default::default();
        self._beacons = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._beacons);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_message_list!(bfr, self._beacons);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._beacons = deserialize_message_list_as::<LblBeacon>(bfr)?;
        Ok(())
    }
}
