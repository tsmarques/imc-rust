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
use crate::VehicleFormationParticipant::VehicleFormationParticipant;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Request
    FC_REQUEST = 0,
    /// Report
    FC_REPORT = 1,
}

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Start
    OP_START = 0,
    /// Stop
    OP_STOP = 1,
    /// Ready
    OP_READY = 2,
    /// Executing
    OP_EXECUTING = 3,
    /// Failure
    OP_FAILURE = 4,
}

/// Formation Reference Frame
#[allow(non_camel_case_types)]
pub enum FormationReferenceFrameEnum {
    /// Earth Fixed
    OP_EARTH_FIXED = 0,
    /// Path Fixed
    OP_PATH_FIXED = 1,
    /// Path Curved
    OP_PATH_CURVED = 2,
}

/// The "Formation" is a controller to execute a maneuver with a team of vehicles.
/// It defines the:
/// - Vehicles included in the formation group
/// - Vehicles relative positions inside the formation
/// - Reference frame where the relative positions are defined
/// - Formation shape configuration
/// - Plan (set of maneuvers) to be followed by the formation center
/// - Plan contrains (virtual leader speed and bank limits)
/// - Supervision settings
/// The formation reference frame may be:
/// - Earth Fixed: Where the vehicles relative position do not depend on the followed path.
/// This results in all UAVs following the same path with an offset relative to each other;
/// - Path Fixed:  Where the vehicles relative position depends on the followed path,
/// changing the inter-vehicle offset direction with the path direction.
/// - Path Curved:  Where the vehicles relative position depends on the followed path,
/// changing the inter-vehicle offset direction with the path direction and direction
/// change rate.
/// An offset in the xx axis results in a distance over the curved path line.
/// An offset in the yy axis results in an offset of the vehicle path line relative to the
/// formation center path line.
#[derive(Default, Clone)]
pub struct Formation {
    /// Message Header
    pub _header: Header,
    /// Name of the formation configuration.
    pub _formation_name: String,
    /// Indicates if the message is a request, or a reply to a previous request.
    pub _type: u8,
    /// Operation to perform.
    pub _op: u8,
    /// Target group for the formation plan.
    pub _group_name: String,
    /// The flight plan's identifier.
    /// Flight plan defined to be tracked by the formation leader.
    pub _plan_id: String,
    /// Verbose text description of plan.
    pub _description: String,
    /// Formation reference frame
    pub _reference_frame: u8,
    /// List of formation participants.
    pub _participants: MessageList<VehicleFormationParticipant>,
    /// Maximum absolute bank allowed for the formation leader.
    pub _leader_bank_lim: f32,
    /// Minimum speed allowed for the formation leader flight.
    pub _leader_speed_min: f32,
    /// Maximum speed allowed for the formation leader flight.
    pub _leader_speed_max: f32,
    /// Minimum altitude allowed for the formation leader flight.
    pub _leader_alt_min: f32,
    /// Maximum altitude allowed for the formation leader flight.
    pub _leader_alt_max: f32,
    /// Limit for the position mismatch between real and simulated position, before a maneuver abort is asserted.
    pub _pos_sim_err_lim: f32,
    /// Warning threshold for the position mismatch between real and simulated position.
    /// Above this threshold a time-out limit is evaluated to assert a maneuver abort state.
    pub _pos_sim_err_wrn: f32,
    /// The amount of time the maneuver is allowed to run after the position mismatch threshold is surpassed.
    pub _pos_sim_err_timeout: u16,
    /// Threshold for the convergence measure, above which a time-out limit
    /// is evaluated to assert a maneuver abort state.
    pub _converg_max: f32,
    /// The amount of time the maneuver is allowed to run after the convergence threshold is surpassed.
    pub _converg_timeout: u16,
    /// The amount of time the maneuver is allowed to run without any update on the other formation vehicles states.
    pub _comms_timeout: u16,
    /// Turbulence limit above which a maneuver abort is asserted.
    pub _turb_lim: f32,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Formation {
    fn new() -> Formation {
        

        Formation {
            _header: Header::new(484),
            _formation_name: Default::default(),
            _type: Default::default(),
            _op: Default::default(),
            _group_name: Default::default(),
            _plan_id: Default::default(),
            _description: Default::default(),
            _reference_frame: Default::default(),
            _participants: Default::default(),
            _leader_bank_lim: Default::default(),
            _leader_speed_min: Default::default(),
            _leader_speed_max: Default::default(),
            _leader_alt_min: Default::default(),
            _leader_alt_max: Default::default(),
            _pos_sim_err_lim: Default::default(),
            _pos_sim_err_wrn: Default::default(),
            _pos_sim_err_timeout: Default::default(),
            _converg_max: Default::default(),
            _converg_timeout: Default::default(),
            _comms_timeout: Default::default(),
            _turb_lim: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        484
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        484
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
        for m in &mut self._participants {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._participants {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._participants {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._participants {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._participants {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(484);
        self._formation_name = Default::default();
        self._type = Default::default();
        self._op = Default::default();
        self._group_name = Default::default();
        self._plan_id = Default::default();
        self._description = Default::default();
        self._reference_frame = Default::default();
        self._participants = Default::default();
        self._leader_bank_lim = Default::default();
        self._leader_speed_min = Default::default();
        self._leader_speed_max = Default::default();
        self._leader_alt_min = Default::default();
        self._leader_alt_max = Default::default();
        self._pos_sim_err_lim = Default::default();
        self._pos_sim_err_wrn = Default::default();
        self._pos_sim_err_timeout = Default::default();
        self._converg_max = Default::default();
        self._converg_timeout = Default::default();
        self._comms_timeout = Default::default();
        self._turb_lim = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        45
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._formation_name.len() + 2;
        dyn_size += self._group_name.len() + 2;
        dyn_size += self._plan_id.len() + 2;
        dyn_size += self._description.len() + 2;
        message_list_serialization_size!(dyn_size, self._participants);
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._formation_name.as_bytes());
        bfr.put_u8(self._type);
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._group_name.as_bytes());
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        serialize_bytes!(bfr, self._description.as_bytes());
        bfr.put_u8(self._reference_frame);
        serialize_message_list!(bfr, self._participants);
        bfr.put_f32_le(self._leader_bank_lim);
        bfr.put_f32_le(self._leader_speed_min);
        bfr.put_f32_le(self._leader_speed_max);
        bfr.put_f32_le(self._leader_alt_min);
        bfr.put_f32_le(self._leader_alt_max);
        bfr.put_f32_le(self._pos_sim_err_lim);
        bfr.put_f32_le(self._pos_sim_err_wrn);
        bfr.put_u16_le(self._pos_sim_err_timeout);
        bfr.put_f32_le(self._converg_max);
        bfr.put_u16_le(self._converg_timeout);
        bfr.put_u16_le(self._comms_timeout);
        bfr.put_f32_le(self._turb_lim);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._formation_name);
        self._type = bfr.get_u8();
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._group_name);
        deserialize_string!(bfr, self._plan_id);
        deserialize_string!(bfr, self._description);
        self._reference_frame = bfr.get_u8();
        self._participants = deserialize_message_list_as::<VehicleFormationParticipant>(bfr)?;
        self._leader_bank_lim = bfr.get_f32_le();
        self._leader_speed_min = bfr.get_f32_le();
        self._leader_speed_max = bfr.get_f32_le();
        self._leader_alt_min = bfr.get_f32_le();
        self._leader_alt_max = bfr.get_f32_le();
        self._pos_sim_err_lim = bfr.get_f32_le();
        self._pos_sim_err_wrn = bfr.get_f32_le();
        self._pos_sim_err_timeout = bfr.get_u16_le();
        self._converg_max = bfr.get_f32_le();
        self._converg_timeout = bfr.get_u16_le();
        self._comms_timeout = bfr.get_u16_le();
        self._turb_lim = bfr.get_f32_le();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
