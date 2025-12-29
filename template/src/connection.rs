use crate::batch_reader::SingleBatchReader;
use crate::statement::AdbcStatement;
use adbc_core::error::{Error, Result, Status};
use adbc_core::options::{InfoCode, ObjectDepth, OptionConnection, OptionValue};
use adbc_core::{Connection, Optionable};
use arrow_array::RecordBatchReader;
use arrow_schema::Schema;
use std::collections::HashSet;

pub struct AdbcConnection {}

impl Connection for AdbcConnection {
    type StatementType = AdbcStatement;

    fn new_statement(&mut self) -> Result<Self::StatementType> {
        Ok(AdbcStatement {})
    }

    fn cancel(&mut self) -> Result<()> {
        Err(Error::with_message_and_status(
            "cancel not implemented",
            Status::NotImplemented,
        ))
    }

    fn get_info(&self, codes: Option<HashSet<InfoCode>>) -> Result<impl RecordBatchReader + Send> {
        match codes {
            _ => Err::<SingleBatchReader, Error>(Error::with_message_and_status(
                "get_info not implemented",
                Status::NotImplemented,
            )),
        }
    }

    fn get_objects(
        &self,
        depth: ObjectDepth,
        catalog: Option<&str>,
        db_schema: Option<&str>,
        table_name: Option<&str>,
        table_type: Option<Vec<&str>>,
        column_name: Option<&str>,
    ) -> Result<impl RecordBatchReader + Send> {
        match (
            depth,
            catalog,
            db_schema,
            table_name,
            table_type,
            column_name,
        ) {
            _ => Err::<SingleBatchReader, Error>(Error::with_message_and_status(
                "get_objects not implemented",
                Status::NotImplemented,
            )),
        }
    }

    fn get_table_schema(
        &self,
        catalog: Option<&str>,
        db_schema: Option<&str>,
        table_name: &str,
    ) -> Result<Schema> {
        match (catalog, db_schema, table_name) {
            _ => Err(Error::with_message_and_status(
                "get_table_schema not implemented",
                Status::NotImplemented,
            )),
        }
    }

    fn get_table_types(&self) -> Result<impl RecordBatchReader + Send> {
        Err::<SingleBatchReader, Error>(Error::with_message_and_status(
            "get_table_types not implemented",
            Status::NotImplemented,
        ))
    }

    fn get_statistic_names(&self) -> Result<impl RecordBatchReader + Send> {
        Err::<SingleBatchReader, Error>(Error::with_message_and_status(
            "get_statistic_names not implemented",
            Status::NotImplemented,
        ))
    }

    fn get_statistics(
        &self,
        catalog: Option<&str>,
        db_schema: Option<&str>,
        table_name: Option<&str>,
        approximate: bool,
    ) -> Result<impl RecordBatchReader + Send> {
        match (catalog, db_schema, table_name, approximate) {
            _ => Err::<SingleBatchReader, Error>(Error::with_message_and_status(
                "get_statistics not implemented",
                Status::NotImplemented,
            )),
        }
    }

    fn commit(&mut self) -> Result<()> {
        Err(Error::with_message_and_status(
            "commit not implemented",
            Status::NotImplemented,
        ))
    }

    fn rollback(&mut self) -> Result<()> {
        Err(Error::with_message_and_status(
            "rollback not implemented",
            Status::NotImplemented,
        ))
    }

    fn read_partition(&self, partition: impl AsRef<[u8]>) -> Result<impl RecordBatchReader + Send> {
        match partition {
            _ => Err::<SingleBatchReader, Error>(Error::with_message_and_status(
                "read_partition not implemented",
                Status::NotImplemented,
            )),
        }
    }
}

impl Optionable for AdbcConnection {
    type Option = OptionConnection;

    fn set_option(&mut self, key: Self::Option, value: OptionValue) -> Result<()> {
        Err(Error::with_message_and_status(
            format!("unsupported connection option {key:?}:{value:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_string(&self, key: Self::Option) -> Result<String> {
        Err(Error::with_message_and_status(
            format!("unsupported connection option {key:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_bytes(&self, key: Self::Option) -> Result<Vec<u8>> {
        Err(Error::with_message_and_status(
            format!("unsupported connection option {key:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_int(&self, key: Self::Option) -> Result<i64> {
        Err(Error::with_message_and_status(
            format!("unsupported connection option {key:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_double(&self, key: Self::Option) -> Result<f64> {
        Err(Error::with_message_and_status(
            format!("unsupported connection option {key:?}"),
            Status::NotImplemented,
        ))
    }
}
