use diesel::result::Error::NotFound;

#[derive(Debug)]
pub enum Error {
    DatabaseError(diesel::result::Error),
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        return Error::DatabaseError(e);
    }
}
