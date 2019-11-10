#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]

pub mod imc;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn all_messages() {
        let mut file = File::create("/tmp/rust.lsf");
    }
}
