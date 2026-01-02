use crate::connection::AdbcConnection;
use crate::error::ErrorHelper;
use adbc_core::error::Result;
use adbc_core::options::{OptionConnection, OptionDatabase, OptionValue};
use adbc_core::{Database, Optionable};
use driverbase::error::ErrorHelper as _;

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
        match (&key, &value) {
            _ => Err(ErrorHelper::set_unknown_option(&key).to_adbc()),
        }
    }

    fn get_option_string(&self, key: Self::Option) -> Result<String> {
        Err(ErrorHelper::set_unknown_option(&key).to_adbc())
    }

    fn get_option_bytes(&self, key: Self::Option) -> Result<Vec<u8>> {
        Err(ErrorHelper::set_unknown_option(&key).to_adbc())
    }

    fn get_option_int(&self, key: Self::Option) -> Result<i64> {
        Err(ErrorHelper::set_unknown_option(&key).to_adbc())
    }

    fn get_option_double(&self, key: Self::Option) -> Result<f64> {
        Err(ErrorHelper::set_unknown_option(&key).to_adbc())
    }
}
