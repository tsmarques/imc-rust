use crate::Header::Header;
use crate::LoggingControl::LoggingControl;
use crate::Message::Message;

pub(crate) fn buildFrom<T: Message>(hdr: Header) -> Option<Box<T>> {
    let mut msg: T = T::new();

    msg.get_header()._mgid = hdr._mgid;
    msg.get_header()._sync = hdr._sync;
    msg.get_header()._size = hdr._size;
    msg.get_header()._timestamp = hdr._timestamp;
    msg.get_header()._src = hdr._src;
    msg.get_header()._src_ent = hdr._src_ent;
    msg.get_header()._dst = hdr._dst;
    msg.get_header()._dst_ent = hdr._dst_ent;

    Option::Some(Box::new(msg))
}

pub(crate) fn build(mgid: u16) -> Option<Box<dyn Message>> {
    match mgid {
        102 => Option::Some(Box::new(LoggingControl::new())),
        _ => Option::None,
    }
}
