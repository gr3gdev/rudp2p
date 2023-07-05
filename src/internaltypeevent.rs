use crate::{Error, InternalTypeEvent};

impl InternalTypeEvent {
    pub(crate) fn get_code(&self) -> u8 {
        match self {
            InternalTypeEvent::CONNECTING => 0,
            InternalTypeEvent::DISCONNECTING => 1,
            InternalTypeEvent::MESSAGE => 2,
            InternalTypeEvent::PEERS => 3,
        }
    }

    pub(crate) fn from_code(code: u8) -> Result<Self, Error> {
        match code {
            0 => Ok(InternalTypeEvent::CONNECTING),
            1 => Ok(InternalTypeEvent::DISCONNECTING),
            2 => Ok(InternalTypeEvent::MESSAGE),
            3 => Ok(InternalTypeEvent::PEERS),
            _ => Err(Error::custom("Unknown event !".to_owned())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::InternalTypeEvent;

    #[test]
    fn test_internal_event() {
        assert_eq!(0, InternalTypeEvent::CONNECTING.get_code());
        assert_eq!(
            InternalTypeEvent::CONNECTING,
            InternalTypeEvent::from_code(0).unwrap()
        );
        assert_eq!(1, InternalTypeEvent::DISCONNECTING.get_code());
        assert_eq!(
            InternalTypeEvent::DISCONNECTING,
            InternalTypeEvent::from_code(1).unwrap()
        );
        assert_eq!(2, InternalTypeEvent::MESSAGE.get_code());
        assert_eq!(
            InternalTypeEvent::MESSAGE,
            InternalTypeEvent::from_code(2).unwrap()
        );
        assert_eq!(3, InternalTypeEvent::PEERS.get_code());
        assert_eq!(
            InternalTypeEvent::PEERS,
            InternalTypeEvent::from_code(3).unwrap()
        );
        assert_eq!(
            "Unknown event !",
            InternalTypeEvent::from_code(4).err().unwrap().message
        );
    }
}
