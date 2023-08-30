use crate::{MessageMethod, Error};

impl MessageMethod {
    pub(crate) fn get_code(&self) -> u8 {
        match self {
            MessageMethod::Request => 0,
            MessageMethod::Response => 1,
            MessageMethod::RequestAndResponse => 2,
            MessageMethod::Other => 3,
        }
    }

    pub(crate) fn from_code(code: u8) -> Result<Self, Error> {
        match code {
            0 => Ok(MessageMethod::Request),
            1 => Ok(MessageMethod::Response),
            2 => Ok(MessageMethod::RequestAndResponse),
            3 => Ok(MessageMethod::Other),
            _ => Err(Error::custom("Unknown method !")),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::MessageMethod;

    #[test]
    fn test_message_method() {
        assert_eq!(0, MessageMethod::Request.get_code());
        assert_eq!(
            MessageMethod::Request,
            MessageMethod::from_code(0).unwrap()
        );
        assert_eq!(1, MessageMethod::Response.get_code());
        assert_eq!(
            MessageMethod::Response,
            MessageMethod::from_code(1).unwrap()
        );
        assert_eq!(2, MessageMethod::RequestAndResponse.get_code());
        assert_eq!(
            MessageMethod::RequestAndResponse,
            MessageMethod::from_code(2).unwrap()
        );
        assert_eq!(3, MessageMethod::Other.get_code());
        assert_eq!(
            MessageMethod::Other,
            MessageMethod::from_code(3).unwrap()
        );
        assert_eq!(
            "Unknown method !",
            MessageMethod::from_code(4).err().unwrap().message
        );
    }
}
