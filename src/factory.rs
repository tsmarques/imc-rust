use crate::Header::Header;
use crate::LoggingControl::LoggingControl;
use crate::Message::Message;
use crate::TrexAttribute::TrexAttribute;

pub(crate) fn buildFrom<T: Message>(hdr: Header) -> Option<Box<T>> {
    let mut msg: T = T::new();

    if msg.id() != hdr._mgid {
        return Option::None;
    }

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

pub(crate) fn build(hdr: Header) -> Option<Box<dyn Message>> {
    let ret = buildFromId(hdr._mgid);
    if ret.is_none() {
        return ret;
    }

    let mut msg = ret.unwrap();
    msg.get_header()._dst = hdr._dst;
    msg.get_header()._size = hdr._size;
    msg.get_header()._sync = hdr._sync;
    msg.get_header()._src = hdr._src;
    msg.get_header()._dst_ent = hdr._dst_ent;
    msg.get_header()._src_ent = hdr._src_ent;
    msg.get_header()._timestamp = hdr._timestamp;

    return Option::from(msg);
}

pub(crate) fn buildFromId(id: u16) -> Option<Box<dyn Message>> {
    match id {
        102 => Option::Some(Box::new(LoggingControl::new())),
        656 => Option::Some(Box::new(TrexAttribute::new())),
        _ => Option::None,
    }
}
