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

use crate::Header::Header;
use crate::Message::*;

/// Action
#[allow(non_camel_case_types)]
pub enum ActionEnum {
    /// Reset Zoom
    ACTION_ZOOM_RESET = 0,
    /// Zoom In
    ACTION_ZOOM_IN = 1,
    /// Zoom Out
    ACTION_ZOOM_OUT = 2,
    /// Stop Zooming
    ACTION_ZOOM_STOP = 3,
}

/// Camera Zoom.
#[derive(Default, Clone)]
pub struct CameraZoom {
    /// Message Header
    pub _header: Header,
    /// The identification number of the destination camera.
    pub _id: u8,
    /// Absolute zoom level.
    pub _zoom: u8,
    /// The zoom action to perform.
    pub _action: u8,
}

impl Message for CameraZoom {
    fn new() -> CameraZoom {
        

        CameraZoom {
            _header: Header::new(300),
            _id: Default::default(),
            _zoom: Default::default(),
            _action: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        300
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        300
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
        self._header = Header::new(300);
        self._id = Default::default();
        self._zoom = Default::default();
        self._action = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        bfr.put_u8(self._zoom);
        bfr.put_u8(self._action);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._id = bfr.get_u8();
        self._zoom = bfr.get_u8();
        self._action = bfr.get_u8();
        Ok(())
    }
}
