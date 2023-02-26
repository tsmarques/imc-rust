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

/// Whenever the CUCS receives a message from one of the existing sensors (through SMS, ZigBee, Acoustic Comms, ...) it disseminates that info recurring to this message.
#[derive(Default, Clone)]
pub struct RemoteSensorInfo {
    /// Message Header
    pub _header: Header,
    /// An unique string that identifies the sensor. Used mostly for logging/presentation.
    pub _id: String,
    /// The class of a sensor tells the type of sensor originating this message. It will determine how the sensor is to be shown and (optionally) how the custom data (tuplelist) is to be interpreted.
    pub _sensor_class: String,
    pub _lat: f64,
    pub _lon: f64,
    pub _alt: f32,
    pub _heading: f32,
    pub _data: String,
}

impl Message for RemoteSensorInfo {
    fn new() -> RemoteSensorInfo {
        

        RemoteSensorInfo {
            _header: Header::new(601),
            _id: Default::default(),
            _sensor_class: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _alt: Default::default(),
            _heading: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        601
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        601
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
        self._header = Header::new(601);
        self._id = Default::default();
        self._sensor_class = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._alt = Default::default();
        self._heading = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        24
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._id.len() + 2;
        dyn_size += self._sensor_class.len() + 2;
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._id.as_bytes());
        serialize_bytes!(bfr, self._sensor_class.as_bytes());
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._alt);
        bfr.put_f32_le(self._heading);
        serialize_bytes!(bfr, self._data.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._id);
        deserialize_string!(bfr, self._sensor_class);
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._alt = bfr.get_f32_le();
        self._heading = bfr.get_f32_le();
        deserialize_string!(bfr, self._data);
        Ok(())
    }
}
