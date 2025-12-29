use crate::database::AdbcDatabase;
use adbc_core::Driver;
use adbc_core::error::Result;
use adbc_core::options::{OptionDatabase, OptionValue};

#[derive(Default)]
pub struct AdbcDriver {}

impl Driver for AdbcDriver {
    type DatabaseType = AdbcDatabase;

    fn new_database(&mut self) -> Result<Self::DatabaseType> {
        self.new_database_with_opts(vec![])
    }

    fn new_database_with_opts(
        &mut self,
        opts: impl IntoIterator<Item = (OptionDatabase, OptionValue)>,
    ) -> Result<Self::DatabaseType> {
        match opts {
            _ => Ok(AdbcDatabase {}),
        }
    }
}
