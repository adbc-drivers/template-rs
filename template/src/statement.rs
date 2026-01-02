use crate::batch_reader::SingleBatchReader;
use crate::error::ErrorHelper;
use adbc_core::error::{Error, Result};
use adbc_core::options::{OptionStatement, OptionValue};
use adbc_core::{Optionable, PartitionedResult, Statement};
use arrow_array::{RecordBatch, RecordBatchReader};
use arrow_schema::Schema;
use driverbase::error::ErrorHelper as _;

pub struct AdbcStatement {}

impl Statement for AdbcStatement {
    fn bind(&mut self, batch: RecordBatch) -> Result<()> {
        match batch {
            _ => Err(ErrorHelper::not_implemented()
                .message("bind not implemented")
                .to_adbc()),
        }
    }

    fn bind_stream(&mut self, reader: Box<dyn RecordBatchReader + Send>) -> Result<()> {
        match reader {
            _ => Err(ErrorHelper::not_implemented()
                .message("bind_stream not implemented")
                .to_adbc()),
        }
    }

    fn execute(&mut self) -> Result<impl RecordBatchReader + Send> {
        Err::<SingleBatchReader, Error>(
            ErrorHelper::not_implemented()
                .message("execute not implemented")
                .to_adbc(),
        )
    }

    fn execute_update(&mut self) -> Result<Option<i64>> {
        Err(ErrorHelper::not_implemented()
            .message("execute_update not implemented")
            .to_adbc())
    }

    fn execute_schema(&mut self) -> Result<Schema> {
        Err(ErrorHelper::not_implemented()
            .message("execute_schema not implemented")
            .to_adbc())
    }

    fn execute_partitions(&mut self) -> Result<PartitionedResult> {
        Err(ErrorHelper::not_implemented()
            .message("execute_partitions not implemented")
            .to_adbc())
    }

    fn get_parameter_schema(&self) -> Result<Schema> {
        Err(ErrorHelper::not_implemented()
            .message("get_parameter_schema not implemented")
            .to_adbc())
    }

    fn prepare(&mut self) -> Result<()> {
        Err(ErrorHelper::not_implemented()
            .message("prepare not implemented")
            .to_adbc())
    }

    fn set_sql_query(&mut self, query: impl AsRef<str>) -> Result<()> {
        match query {
            _ => Err(ErrorHelper::not_implemented()
                .message("set_sql_query not implemented")
                .to_adbc()),
        }
    }

    fn set_substrait_plan(&mut self, plan: impl AsRef<[u8]>) -> Result<()> {
        match plan {
            _ => Err(ErrorHelper::not_implemented()
                .message("set_substrait_plan not implemented")
                .to_adbc()),
        }
    }

    fn cancel(&mut self) -> Result<()> {
        Err(ErrorHelper::not_implemented()
            .message("cancel not implemented")
            .to_adbc())
    }
}

impl Optionable for AdbcStatement {
    type Option = OptionStatement;

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
