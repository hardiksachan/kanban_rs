use super::Error;

use derive_more::From;
use serde::Serialize;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, From, Clone, PartialEq, Serialize)]
pub struct TicketId(Uuid);

impl TicketId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for TicketId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let encoded = bs58::encode(self.0.as_bytes()).into_string();
        write!(fmt, "{}", encoded)
    }
}

impl TryFrom<String> for TicketId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let decoded = bs58::decode(value.clone())
            .into_vec()
            .map_err(|_| Error::InvalidTicketId { id: value.clone() })?;

        match Uuid::try_from(decoded) {
            Err(_) => Err(Error::InvalidTicketId { id: value.clone() }),
            Ok(id) => Ok(id.into()),
        }
    }
}

#[derive(Debug, From, Clone, PartialEq)]
pub struct OwnerId(String);

impl Display for OwnerId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Clone, Debug, From)]
pub struct Title(String);

impl Title {
    pub fn get(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Ticket {
    ticket_id: TicketId,
    owner_id: OwnerId,
    title: Title,
}

impl Ticket {
    pub fn new(owner_id: OwnerId, title: Title) -> Ticket {
        Ticket {
            ticket_id: TicketId::new(),
            owner_id,
            title,
        }
    }
}

impl Ticket {
    pub fn ticket_id(&self) -> &TicketId {
        &self.ticket_id
    }
    pub fn owner_id(&self) -> &OwnerId {
        &self.owner_id
    }
    pub fn title(&self) -> &Title {
        &self.title
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ticket_id_display_uses_base58() {
        let id: TicketId = Uuid::parse_str("3f1cfc41-c05f-4e8d-97e5-24f2261859ae")
            .unwrap()
            .into();
        assert_eq!(id.to_string(), "8o2JRD9CYxLmzTpCdzSZTK".to_string());
    }

    #[test]
    fn ticket_id_parse_from_base58_successful() {
        let id_from_str: TicketId = Uuid::parse_str("3f1cfc41-c05f-4e8d-97e5-24f2261859ae")
            .unwrap()
            .into();
        let id_from_base58: TicketId = "8o2JRD9CYxLmzTpCdzSZTK".to_owned().try_into().unwrap();
        assert_eq!(id_from_str, id_from_base58);
    }
}
