use std::io;

#[derive(Debug)]
pub struct Message {
    pub content: io::Result<Option<Vec<u8>>>,
}

impl Message {
    pub fn parse_next_line(length: io::Result<usize>, response: Vec<u8>) -> Self {
        Self {
            content: match length {
                Ok(length) if length > 1 => Ok(Some(Self::trunc_message(response))),
                Ok(_) => Ok(None),
                Err(err) => Err(err),
            },
        }
    }

    fn trunc_message(mut message: Vec<u8>) -> Vec<u8> {
        if message.last().is_some_and(|byte| byte == &b'\n') {
            message.pop();
            if message.last().is_some_and(|byte| byte == &b'\r') {
                message.pop();
            }
        }
        message
    }
}
