use crate::Message::Message;
use std::option::Iter;
use std::slice::IterMut;

#[derive(Default)]
pub struct MessageList<T: ?Sized>
where
    T: Message,
{
    _list: Vec<Option<Box<T>>>,
}

impl<T> MessageList<T>
where
    T: Message,
{
    pub fn iter_mut(&mut self) -> IterMut<'_, Option<Box<T>>> {
        self._list.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<Box<T>>> {
        self._list.iter()
    }

    pub fn clear(&mut self) {
        for msg in self._list.iter_mut() {
            match msg {
                None => {}
                Some(f) => {
                    f.clear();
                }
            }
        }
    }

    pub fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        for msg in self._list.iter() {
            match msg {
                None => {}
                Some(f) => {
                    f.serialize_fields(bfr);
                }
            }
        }
    }

    pub fn dynamic_serialization_size(&self) -> usize {
        let mut size = 0;
        for msg in self._list.iter() {
            match msg {
                None => {}
                Some(f) => size += f.dynamic_serialization_size(),
            }
        }

        size
    }
}
