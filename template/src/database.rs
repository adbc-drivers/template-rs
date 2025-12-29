use crate::connection::AdbcConnection;
use adbc_core::error::{Error, Result, Status};
use adbc_core::options::{OptionConnection, OptionDatabase, OptionValue};
use adbc_core::{Database, Optionable};

pub struct AdbcDatabase {}

impl Database for AdbcDatabase {
    type ConnectionType = AdbcConnection;

    fn new_connection(&self) -> Result<Self::ConnectionType> {
        self.new_connection_with_opts(vec![])
    }

    fn new_connection_with_opts(
        &self,
        opts: impl IntoIterator<Item = (OptionConnection, OptionValue)>,
    ) -> Result<Self::ConnectionType> {
        match opts {
            _ => Ok(AdbcConnection {}),
        }
    }
}

impl Optionable for AdbcDatabase {
    type Option = OptionDatabase;

    fn set_option(&mut self, key: Self::Option, value: OptionValue) -> Result<()> {
        Err(Error::with_message_and_status(
            format!("unsupported database option {key:?}:{value:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_string(&self, key: Self::Option) -> Result<String> {
        Err(Error::with_message_and_status(
            format!("unsupported database option {key:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_bytes(&self, key: Self::Option) -> Result<Vec<u8>> {
        Err(Error::with_message_and_status(
            format!("unsupported database option {key:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_int(&self, key: Self::Option) -> Result<i64> {
        Err(Error::with_message_and_status(
            format!("unsupported database option {key:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_double(&self, key: Self::Option) -> Result<f64> {
        Err(Error::with_message_and_status(
            format!("unsupported database option {key:?}"),
            Status::NotImplemented,
        ))
    }
}
