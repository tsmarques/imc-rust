#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]

pub mod imc;

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use crate::imc::*;
    use crate::imc::Heartbeat::Heartbeat;
    use crate::imc::LoggingControl::{LoggingControl, ControlOperationEnum};
    use std::path::PathBuf;
    use crate::imc::Message::Message;
    use bytes::{BufMut, BytesMut};

    #[test]
    fn all_messages() {
        let temp_directory: PathBuf = env::temp_dir();
        let file_path = temp_directory.join("rust.lsf");
        let mut file = File::create(file_path).unwrap();

        let mut bfr :bytes::BytesMut = BytesMut::with_capacity(4096);;

        let mut log_control:LoggingControl = LoggingControl::new();
        log_control.name = "rust-test";
        log_control.op = ControlOperationEnum::COP_REQUEST_START;

        log_control.serialize(&mut bfr);
//        writeln!(&mut file, bfr).unwrap();
        file.write_all(bfr.as_ref()).unwrap();
    }
}
