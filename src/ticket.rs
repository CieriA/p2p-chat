use {
    data_encoding::BASE32_NOPAD,
    serde::{Deserialize, Serialize},
    std::{error::Error, fmt, net::SocketAddr, str::FromStr},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub peers: Vec<SocketAddr>,
}

impl Ticket {
    #[inline]
    pub const fn new(id: String, peers: Vec<SocketAddr>) -> Self {
        Self { id, peers }
    }
}
impl TryFrom<&[u8]> for Ticket {
    type Error = Box<dyn Error>;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let ticket = serde_json::from_slice(bytes)?;
        Ok(ticket)
    }
}
impl From<&Ticket> for Vec<u8> {
    fn from(ticket: &Ticket) -> Self {
        serde_json::to_vec(ticket).unwrap()
    }
}
impl FromStr for Ticket {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = BASE32_NOPAD.decode(s.to_ascii_uppercase().as_bytes())?;
        bytes.as_slice().try_into()
    }
}
impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: Vec<u8> = self.into();
        let text = BASE32_NOPAD.encode(&bytes).to_ascii_lowercase();
        write!(f, "{text}")
    }
}
