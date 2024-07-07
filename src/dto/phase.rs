#[derive(Debug)]
pub enum ListenPhase {
    Init = 0,
    Tunneling = 1,
    Heartbeat = 99,
}

impl TryFrom<i32> for ListenPhase {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == ListenPhase::Init as i32 => Ok(ListenPhase::Init),
            x if x == ListenPhase::Tunneling as i32 => Ok(ListenPhase::Tunneling),
            x if x == ListenPhase::Heartbeat as i32 => Ok(ListenPhase::Heartbeat),
            _ => Err(()),
        }
    }
}
