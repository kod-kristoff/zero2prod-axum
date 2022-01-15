use std::{fmt, io};

use diesel::sql_types::Binary;

use uuid;

#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, Hash, Eq, PartialEq, serde::Serialize)]
#[sql_type = "Binary"]
pub struct Uuid(pub uuid::Uuid);

impl Uuid {
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(s: Uuid) -> Self {
        s.0
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<B: diesel::backend::Backend> diesel::deserialize::FromSql<Binary, B> for Uuid
where
    Vec<u8>: diesel::deserialize::FromSql<Binary, B>,
{
    fn from_sql(
        bytes: Option<&B::RawValue>
    ) -> diesel::deserialize::Result<Self> {
        let value = <Vec<u8>>::from_sql(bytes)?;
        uuid::Uuid::from_slice(&value)
            .map(Uuid)
            .map_err(|e| e.into())
    }
}

impl<B: diesel::backend::Backend> diesel::serialize::ToSql<Binary, B> for Uuid
where
    [u8]: diesel::serialize::ToSql<Binary, B>,
{
    fn to_sql<W: io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, B>,
    ) -> diesel::serialize::Result {
        out.write_all(self.0.as_bytes())
            .map(|_| diesel::serialize::IsNull::No)
            .map_err(Into::into)
    }
}
