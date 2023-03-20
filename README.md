# IMC Rust

## Deserialization example

```rust

fn imc_is<T :Message>(id :u16) -> bool {
    T::static_id() == id
}

while let Ok(m) = imc::packet::deserialize(&mut data) {
    if imc_is::<imc::EstimatedState::EstimatedState>(m.id()) {
        let estate =  m.as_any().downcast_ref::<imc::EstimatedState::EstimatedState>().unwrap();
        // (...)
    } else if imc_is::<imc::EntityState::EntityState>(m.id()) {
        let entity_state = m.as_any().downcast_ref::<imc::EntityState::EntityState>().unwrap();
        // (...)
    }
}

```
where **data** is of type `dyn bytes::Buf`
