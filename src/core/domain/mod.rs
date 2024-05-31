// Question: How do I force immutability for ticket
// Do I really need it or just need some backing fields?

// These domain objects may later enfoce some invariants
// hence, we don't have primitive types by default

#[derive(Clone, Debug)]
pub struct TicketId(u64);

impl TicketId {
    pub fn get(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct OwnerId(u64);

impl From<u64> for OwnerId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl OwnerId {
    pub fn get(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct Title(String);

impl From<String> for Title {
    fn from(value: String) -> Self {
        Self(value)
    }
}

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

// Constructors
impl Ticket {
    pub fn new(owner_id: OwnerId, title: Title) -> Ticket {
        Ticket {
            ticket_id: TicketId(5),
            owner_id,
            title,
        }
    }
}

// Getters
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
