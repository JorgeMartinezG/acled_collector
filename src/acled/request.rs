use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct Request<'a, 'b> {
    pub key: &'a str,
    pub email: &'a str,
    pub page: u8,
    pub iso: u16,
    pub event_date: &'b str,
    pub event_date_where: &'b str,
}

impl<'a, 'b> Serialize for Request<'a, 'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Request", 4)?;
        s.serialize_field("key", &self.key)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("page", &self.page)?;
        s.serialize_field("iso", &self.iso)?;
        s.serialize_field("event_date", &self.event_date)?;
        s.serialize_field("event_date_where", &self.event_date_where)?;
        s.end()
    }
}
