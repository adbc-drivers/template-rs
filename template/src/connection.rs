use crate::batch_reader::SingleBatchReader;
use crate::error::ErrorHelper;
use crate::statement::AdbcStatement;
use adbc_core::error::{Error, Result};
use adbc_core::options::{InfoCode, ObjectDepth, OptionConnection, OptionValue};
use adbc_core::{Connection, Optionable};
use arrow_array::RecordBatchReader;
use arrow_schema::Schema;
use driverbase::error::ErrorHelper as _;
use std::collections::HashSet;

pub struct AdbcConnection {}

impl Connection for AdbcConnection {
    type StatementType = AdbcStatement;

    fn new_statement(&mut self) -> Result<Self::StatementType> {
        Ok(AdbcStatement {})
    }

    fn cancel(&mut self) -> Result<()> {
        Err(ErrorHelper::not_implemented()
            .message("cancel not implemented")
            .to_adbc())
    }

    fn get_info(&self, codes: Option<HashSet<InfoCode>>) -> Result<impl RecordBatchReader + Send> {
        match codes {
            _ => Err::<SingleBatchReader, Error>(
                ErrorHelper::not_implemented()
                    .message("get_info not implemented")
                    .to_adbc(),
            ),
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
            _ => Err::<SingleBatchReader, Error>(
                ErrorHelper::not_implemented()
                    .message("get_objects not implemented")
                    .to_adbc(),
            ),
        }
    }

    fn get_table_schema(
        &self,
        catalog: Option<&str>,
        db_schema: Option<&str>,
        table_name: &str,
    ) -> Result<Schema> {
        match (catalog, db_schema, table_name) {
            _ => Err(ErrorHelper::not_implemented()
                .message("get_table_schema not implemented")
                .to_adbc()),
        }
    }

    fn get_table_types(&self) -> Result<impl RecordBatchReader + Send> {
        Err::<SingleBatchReader, Error>(
            ErrorHelper::not_implemented()
                .message("get_table_types not implemented")
                .to_adbc(),
        )
    }

    fn get_statistic_names(&self) -> Result<impl RecordBatchReader + Send> {
        Err::<SingleBatchReader, Error>(
            ErrorHelper::not_implemented()
                .message("get_statistic_names not implemented")
                .to_adbc(),
        )
    }

    fn get_statistics(
        &self,
        catalog: Option<&str>,
        db_schema: Option<&str>,
        table_name: Option<&str>,
        approximate: bool,
    ) -> Result<impl RecordBatchReader + Send> {
        match (catalog, db_schema, table_name, approximate) {
            _ => Err::<SingleBatchReader, Error>(
                ErrorHelper::not_implemented()
                    .message("get_statistics not implemented")
                    .to_adbc(),
            ),
        }
    }

    fn commit(&mut self) -> Result<()> {
        Err(ErrorHelper::not_implemented()
            .message("commit not implemented")
            .to_adbc())
    }

    fn rollback(&mut self) -> Result<()> {
        Err(ErrorHelper::not_implemented()
            .message("rollback not implemented")
            .to_adbc())
    }

    fn read_partition(&self, partition: impl AsRef<[u8]>) -> Result<impl RecordBatchReader + Send> {
        match partition {
            _ => Err::<SingleBatchReader, Error>(
                ErrorHelper::not_implemented()
                    .message("read_partition not implemented")
                    .to_adbc(),
            ),
        }
    }
}

impl Optionable for AdbcConnection {
    type Option = OptionConnection;

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
