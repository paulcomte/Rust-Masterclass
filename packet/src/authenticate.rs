use std::io::Cursor;

use prost::Message;

// Include the `authenticate` module, which is generated from autenticate.proto.
include!(concat!(env!("OUT_DIR"), "/packets.authenticate.rs"));

pub fn create<S: Into<String>>(public_token: S) -> Authenticate {
    Authenticate {
        public_token: public_token.into(),
    }
}

pub fn deserialize(buf: &[u8]) -> Result<Authenticate, prost::DecodeError> {
    Authenticate::decode(&mut Cursor::new(buf))
}
