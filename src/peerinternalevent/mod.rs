use crate::{PeerError, PeerInternalEvent};

pub(crate) mod peerconnectedinfo;
pub(crate) mod peerconnectinginfo;
pub(crate) mod peerdisconnectedinfo;
pub(crate) mod peerdisconnectinginfo;
pub(crate) mod peermessageinfo;

impl PeerInternalEvent {
    pub(crate) fn get_code(&self) -> u8 {
        match self {
            PeerInternalEvent::CONNECTING => 1,
            PeerInternalEvent::CONNECTED => 2,
            PeerInternalEvent::MESSAGE => 3,
            PeerInternalEvent::DISCONNECTING => 8,
            PeerInternalEvent::DISCONNECTED => 9,
        }
    }

    pub(crate) fn from_code(code: u8) -> Result<PeerInternalEvent, PeerError> {
        match code {
            1 => Ok(PeerInternalEvent::CONNECTING),
            2 => Ok(PeerInternalEvent::CONNECTED),
            3 => Ok(PeerInternalEvent::MESSAGE),
            8 => Ok(PeerInternalEvent::DISCONNECTING),
            9 => Ok(PeerInternalEvent::DISCONNECTED),
            _ => Err(PeerError::new("Unknown event")),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::PeerInternalEvent;

    #[test]
    fn test_from_code() {
        assert_eq!(
            PeerInternalEvent::CONNECTING,
            PeerInternalEvent::from_code(1).unwrap()
        );
        assert_eq!(
            PeerInternalEvent::CONNECTED,
            PeerInternalEvent::from_code(2).unwrap()
        );
        assert_eq!(
            PeerInternalEvent::MESSAGE,
            PeerInternalEvent::from_code(3).unwrap()
        );
        assert_eq!(
            PeerInternalEvent::DISCONNECTING,
            PeerInternalEvent::from_code(8).unwrap()
        );
        assert_eq!(
            PeerInternalEvent::DISCONNECTED,
            PeerInternalEvent::from_code(9).unwrap()
        );
    }

    #[test]
    fn test_invalid_code() {
        assert!(PeerInternalEvent::from_code(0).is_err());
    }
}
