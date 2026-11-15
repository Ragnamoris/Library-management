#[derive(Debug, Clone, Copy)]
pub struct PublicationYear(i32);

#[derive(Debug, thiserror::Error)]
pub enum PublicationYearError {
    #[error("publication year is not positive")]
    YearIsNotPositive,
    #[error("publication year is in future")]
    YearIsInFuture,
}

impl PublicationYear {
    pub fn new(value: i32) -> Result<Self, PublicationYearError> {
        if value <= 0 {
            return Err(PublicationYearError::YearIsNotPositive);
        }
        if value > 2100 {
            return Err(PublicationYearError::YearIsInFuture);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
