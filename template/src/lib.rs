mod batch_reader;
mod connection;
mod database;
mod driver;
mod error;
mod statement;

use adbc_ffi::export_driver;

export_driver!(AdbcDriver{{ database-name | replace: " ", "" }}Init, driver::AdbcDriver);
