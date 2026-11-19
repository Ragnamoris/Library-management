use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, Value, ValueRef},
    ToSql,
};
use time::{format_description::well_known::Rfc3339, UtcDateTime};

use crate::entities::value_objects::{
    Address, CardNumber, EntityId, InventoryNumber, Name, PhoneNumber, PublicationYear,
    RecordLifetime,
};

// ============= EntityId =============

impl From<EntityId> for Value {
    fn from(value: EntityId) -> Self {
        value.value().into()
    }
}

impl ToSql for EntityId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(self.value().into()))
    }
}

impl FromSql for EntityId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let uuid: uuid::Uuid = uuid::Uuid::column_result(value)?;
        EntityId::create(uuid).map_err(|err| FromSqlError::Other(Box::new(err)))
    }
}

// ============= Name =============

impl From<Name> for Value {
    fn from(value: Name) -> Self {
        value.value().to_string().into()
    }
}

impl ToSql for Name {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(
            self.value().as_bytes(),
        )))
    }
}

impl FromSql for Name {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text: String = String::column_result(value)?;
        Name::new(text).map_err(|err| FromSqlError::Other(Box::new(err)))
    }
}

// ============= Address =============

impl From<Address> for Value {
    fn from(value: Address) -> Self {
        value.value().to_string().into()
    }
}

impl ToSql for Address {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(
            self.value().as_bytes(),
        )))
    }
}

impl FromSql for Address {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text: String = String::column_result(value)?;
        Address::new(text).map_err(|err| FromSqlError::Other(Box::new(err)))
    }
}

// ============= PhoneNumber =============

impl From<PhoneNumber> for Value {
    fn from(value: PhoneNumber) -> Self {
        value.value().to_string().into()
    }
}

impl ToSql for PhoneNumber {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(
            self.value().as_bytes(),
        )))
    }
}

impl FromSql for PhoneNumber {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text: String = String::column_result(value)?;
        PhoneNumber::new(text).map_err(|err| FromSqlError::Other(Box::new(err)))
    }
}

// ============= CardNumber =============

impl From<CardNumber> for Value {
    fn from(value: CardNumber) -> Self {
        value.value().to_string().into()
    }
}

impl ToSql for CardNumber {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(
            self.value().as_bytes(),
        )))
    }
}

impl FromSql for CardNumber {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text: String = String::column_result(value)?;
        CardNumber::new(text).map_err(|err| FromSqlError::Other(Box::new(err)))
    }
}

// ============= InventoryNumber =============

impl From<InventoryNumber> for Value {
    fn from(value: InventoryNumber) -> Self {
        value.value().to_string().into()
    }
}

impl ToSql for InventoryNumber {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(
            self.value().as_bytes(),
        )))
    }
}

impl FromSql for InventoryNumber {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text: String = String::column_result(value)?;
        InventoryNumber::new(text).map_err(|err| FromSqlError::Other(Box::new(err)))
    }
}

// ============= PublicationYear =============

impl From<PublicationYear> for Value {
    fn from(value: PublicationYear) -> Self {
        value.value().into()
    }
}

impl ToSql for PublicationYear {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(self.value() as i64)))
    }
}

impl FromSql for PublicationYear {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let year: i32 = i32::column_result(value)?;
        PublicationYear::new(year).map_err(|err| FromSqlError::Other(Box::new(err)))
    }
}

// ============= RecordLifetime =============

pub fn parse_utc_datetime(
    row: &rusqlite::Row,
    index: usize,
) -> Result<UtcDateTime, rusqlite::Error> {
    let text: String = row.get(index)?;
    UtcDateTime::parse(&text, &Rfc3339).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            index,
            rusqlite::types::Type::Text,
            Box::new(error),
        )
    })
}

pub fn parse_optional_utc_datetime(
    row: &rusqlite::Row,
    index: usize,
) -> Result<Option<UtcDateTime>, rusqlite::Error> {
    if matches!(row.get_ref(index)?, ValueRef::Null) {
        return Ok(None);
    }
    parse_utc_datetime(row, index).map(Some)
}

pub fn parse_record_lifetime(
    row: &rusqlite::Row,
    created_at_index: usize,
    updated_at_index: usize,
    deleted_at_index: usize,
) -> Result<RecordLifetime, rusqlite::Error> {
    let created_at = parse_utc_datetime(row, created_at_index)?;
    let updated_at = parse_optional_utc_datetime(row, updated_at_index)?;
    let deleted_at = parse_optional_utc_datetime(row, deleted_at_index)?;

    RecordLifetime::create(created_at, updated_at, deleted_at).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            created_at_index,
            rusqlite::types::Type::Text,
            Box::new(error),
        )
    })
}
