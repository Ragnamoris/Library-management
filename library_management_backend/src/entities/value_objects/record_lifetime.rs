use time::UtcDateTime;

const MIN_VALID_UNIX_TIMESTAMP: i64 = 946_684_800;
const MAX_VALID_UNIX_TIMESTAMP: i64 = 2_840_140_800;

#[derive(Debug, Clone, Copy)]
pub struct RecordLifetime {
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

#[derive(Debug, thiserror::Error)]
pub enum RecordLifetimeError {
    #[error("record was updated before it was created")]
    UpdatedBeforeCreated,
    #[error("record was deleted before it was created")]
    DeletedBeforeCreated,
    #[error("datetime is out of valid range")]
    DateTimeOutOfRange,
}

impl RecordLifetime {
    pub fn create(
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, RecordLifetimeError> {
        if let Some(updated_at) = updated_at {
            if updated_at < created_at {
                return Err(RecordLifetimeError::UpdatedBeforeCreated);
            }
        }
        if let Some(deleted_at) = deleted_at {
            if deleted_at < created_at {
                return Err(RecordLifetimeError::DeletedBeforeCreated);
            }
        }

        if created_at.unix_timestamp() < MIN_VALID_UNIX_TIMESTAMP
            || created_at.unix_timestamp() > MAX_VALID_UNIX_TIMESTAMP
        {
            return Err(RecordLifetimeError::DateTimeOutOfRange);
        }
        if let Some(updated_at) = updated_at {
            if updated_at.unix_timestamp() < MIN_VALID_UNIX_TIMESTAMP
                || updated_at.unix_timestamp() > MAX_VALID_UNIX_TIMESTAMP
            {
                return Err(RecordLifetimeError::DateTimeOutOfRange);
            }
        }
        if let Some(deleted_at) = deleted_at {
            if deleted_at.unix_timestamp() < MIN_VALID_UNIX_TIMESTAMP
                || deleted_at.unix_timestamp() > MAX_VALID_UNIX_TIMESTAMP
            {
                return Err(RecordLifetimeError::DateTimeOutOfRange);
            }
        }

        Ok(Self {
            created_at,
            updated_at,
            deleted_at,
        })
    }

    pub fn create_new() -> Result<Self, RecordLifetimeError> {
        Self::create(UtcDateTime::now(), None, None)
    }

    pub fn created_at(&self) -> UtcDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<UtcDateTime> {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<UtcDateTime> {
        self.deleted_at
    }
}
