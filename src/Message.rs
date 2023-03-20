#![macro_use]

use crate::packet::ImcError;
use crate::Header::Header;
use crate::{IMC_CONST_FOOTER_SIZE, IMC_CONST_HEADER_SIZE};
use std::any::Any;

macro_rules! serialize_bytes {
    ($bfr:expr, $bytes_slice:expr) => {
        $bfr.put_u16_le($bytes_slice.len() as u16);
        $bfr.put_slice($bytes_slice);
    };
}

macro_rules! message_list_serialization_size {
    ($size:expr, $target_var:expr) => {
        // list size
        $size += 2;
        for m in $target_var.iter() {
            // message id
            $size += 2;
            // message payload
            $size += m.payload_serialization_size();
        }
    };
}

macro_rules! inline_message_serialization_size {
    ($size:expr, $target_var:expr) => {
        $size += 2;
        match &$target_var {
            None => {}
            Some(msg) => {
                $size += msg.payload_serialization_size();
            }
        }
    };
}

macro_rules! serialize_inline_message {
    ($bfr:expr, $target_var:expr) => {
        // inline message
        match &$target_var {
            None => {
                $bfr.put_u16_le(DUNE_IMC_CONST_NULL_ID);
            }

            Some(m) => {
                $bfr.put_u16_le(m.id());
                m.serialize_fields($bfr)
            }
        };
    };
}

macro_rules! serialize_message_list {
    ($bfr:expr, $target_var:expr) => {
        $bfr.put_u16_le($target_var.len() as u16);
        for m in $target_var.iter() {
            // @todo serialize null inline message?
            $bfr.put_u16_le(DUNE_IMC_CONST_NULL_ID);
            m.serialize_fields($bfr);
        }
    };
}

macro_rules! deserialize_string {
    ($bfr:expr, $target_var:expr) => {
        let size = $bfr.get_u16_le();
        for _ in 0..size {
            $target_var.push(char::from($bfr.get_u8()));
        }
    };
}

macro_rules! deserialize_bytes {
    ($bfr:expr, $target_var:expr) => {
        let size = $bfr.get_u16_le();
        for _ in 0..size {
            $target_var.push($bfr.get_u8());
        }
    };
}

/// Basic IMC Message
/// @todo nested callbacks
pub trait Message: MessageClone {
    /// Default constructor
    fn new() -> Self
    where
        Self: Sized;

    /// Retrieve message's identification number.
    fn static_id() -> u16
    where
        Self: Sized;

    /// Retrieve message's identification number.
    fn id(&self) -> u16;

    /// Boiler plate for down cast
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Get a reference to this message
    /// header
    fn get_header(&self) -> &Header;

    /// Get a mutable reference to this message
    /// header
    fn get_mut_header(&mut self) -> &mut Header;

    /// Get the message's time stamp to a given value
    fn get_timestamp_secs(&self) -> f64 {
        self.get_header()._timestamp
    }

    /// Get message's source
    fn get_source(&self) -> u16 {
        self.get_header()._src
    }

    /// Get message's source entity
    fn get_source_ent(&self) -> u8 {
        self.get_header()._src_ent
    }

    /// Get message's destination
    fn get_destination(&self) -> u16 {
        self.get_header()._dst
    }

    /// Get message' destination entity
    fn get_destination_ent(&self) -> u8 {
        self.get_header()._dst_ent
    }

    /// Set the message's time stamp to a given value
    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_mut_header()._timestamp = ts
    }

    /// Set message's source
    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
    }

    /// Set message's source entity
    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
    }

    /// Set message's destination
    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
    }

    /// Set message' destination entity
    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
    }

    // Clear message's fields
    fn clear(&mut self);

    /// Get message's fixed serialization size
    fn fixed_serialization_size(&self) -> usize;

    /// Get message's dynamic serializarion size
    fn dynamic_serialization_size(&self) -> usize;

    /// Serialize this message's fields
    fn serialize_fields(&self, bfr: &mut bytes::BytesMut);

    /// Deserialize this message's fields
    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError>;

    /// Get message fields' serialization size
    /// Fixed + dynamic
    fn payload_serialization_size(&self) -> usize {
        self.fixed_serialization_size() + self.dynamic_serialization_size()
    }

    /// Get complete message's serialization size
    /// Accounts header and footer size as well
    fn serialization_size(&self) -> usize {
        self.payload_serialization_size()
            + IMC_CONST_HEADER_SIZE as usize
            + IMC_CONST_FOOTER_SIZE as usize
    }
}

/// Rust dark magic to allow having "dyn Message" as a field
/// on a struct that derives Clone (e.g. AcousticOperation.rs)
pub trait MessageClone {
    fn do_clone(&self) -> Box<dyn Message>;
}

impl<T> MessageClone for T
where
    T: 'static + Message + Clone,
{
    fn do_clone(&self) -> Box<dyn Message> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Message> {
    fn clone(&self) -> Box<dyn Message> {
        self.do_clone()
    }
}
