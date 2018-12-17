extern crate nom;

#[cfg(test)]
extern crate hex;

pub mod protocol;

#[cfg(test)]
mod tests {
    use hex;
    use protocol::*;

    #[test]
    fn test_local_name() {
        let msg = hex::decode("\
        011300f84368726f6d654c696e75785f37354632000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000").unwrap();

        HciMessage::parse(&msg).unwrap();
    }

    #[test]
    fn test_unknown_command() {
        let msg = hex::decode("040e0601120c000000").unwrap();

        HciMessage::parse(&msg).unwrap();
    }

    #[test]
    fn test_le_advertising_report() {
        let data = "04 3e 28 02 01 02 01 32 9c f0 06 23 68 1c 03 03\
                         9f fe 17 16 9f fe 00 00 00 00 00 00 00 00 00 00\
                         00 00 00 00 00 00 00 00 00 00 b0";

        let msg = hex::decode(data.replace(" ", "")).unwrap();

        HciMessage::parse(&msg).unwrap();
    }
}

