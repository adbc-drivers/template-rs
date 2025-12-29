use crate::batch_reader::SingleBatchReader;
use adbc_core::error::{Error, Result, Status};
use adbc_core::options::{OptionStatement, OptionValue};
use adbc_core::{Optionable, PartitionedResult, Statement};
use arrow_array::{RecordBatch, RecordBatchReader};
use arrow_schema::Schema;

pub struct AdbcStatement {}

impl Statement for AdbcStatement {
    fn bind(&mut self, batch: RecordBatch) -> Result<()> {
        match batch {
            _ => Err(Error::with_message_and_status(
                "bind not implemented",
                Status::NotImplemented,
            )),
        }
    }

    fn bind_stream(&mut self, reader: Box<dyn RecordBatchReader + Send>) -> Result<()> {
        match reader {
            _ => Err(Error::with_message_and_status(
                "bind_stream not implemented",
                Status::NotImplemented,
            )),
        }
    }

    fn execute(&mut self) -> Result<impl RecordBatchReader + Send> {
        Err::<SingleBatchReader, Error>(Error::with_message_and_status(
            "execute not implemented",
            Status::NotImplemented,
        ))
    }

    fn execute_update(&mut self) -> Result<Option<i64>> {
        Err(Error::with_message_and_status(
            "execute_update not implemented",
            Status::NotImplemented,
        ))
    }

    fn execute_schema(&mut self) -> Result<Schema> {
        Err(Error::with_message_and_status(
            "execute_schema not implemented",
            Status::NotImplemented,
        ))
    }

    fn execute_partitions(&mut self) -> Result<PartitionedResult> {
        Err(Error::with_message_and_status(
            "execute_partitions not implemented",
            Status::NotImplemented,
        ))
    }

    fn get_parameter_schema(&self) -> Result<Schema> {
        Err(Error::with_message_and_status(
            "get_parameter_schema not implemented",
            Status::NotImplemented,
        ))
    }

    fn prepare(&mut self) -> Result<()> {
        Err(Error::with_message_and_status(
            "prepare not implemented",
            Status::NotImplemented,
        ))
    }

    fn set_sql_query(&mut self, query: impl AsRef<str>) -> Result<()> {
        match query {
            _ => Err(Error::with_message_and_status(
                "set_sql_query not implemented",
                Status::NotImplemented,
            )),
        }
    }

    fn set_substrait_plan(&mut self, plan: impl AsRef<[u8]>) -> Result<()> {
        match plan {
            _ => Err(Error::with_message_and_status(
                "set_substrait_plan not implemented",
                Status::NotImplemented,
            )),
        }
    }

    fn cancel(&mut self) -> Result<()> {
        Err(Error::with_message_and_status(
            "cancel not implemented",
            Status::NotImplemented,
        ))
    }
}

impl Optionable for AdbcStatement {
    type Option = OptionStatement;

    fn set_option(&mut self, key: Self::Option, value: OptionValue) -> Result<()> {
        Err(Error::with_message_and_status(
            format!("unsupported statement option {key:?}:{value:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_string(&self, key: Self::Option) -> Result<String> {
        Err(Error::with_message_and_status(
            format!("unsupported statement option {key:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_bytes(&self, key: Self::Option) -> Result<Vec<u8>> {
        Err(Error::with_message_and_status(
            format!("unsupported statement option {key:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_int(&self, key: Self::Option) -> Result<i64> {
        Err(Error::with_message_and_status(
            format!("unsupported statement option {key:?}"),
            Status::NotImplemented,
        ))
    }

    fn get_option_double(&self, key: Self::Option) -> Result<f64> {
        Err(Error::with_message_and_status(
            format!("unsupported statement option {key:?}"),
            Status::NotImplemented,
        ))
    }
}
