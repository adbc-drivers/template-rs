#[derive(Clone, Copy, Debug)]
pub struct ErrorHelper {}

impl driverbase::error::ErrorHelper for ErrorHelper {
    const NAME: &'static str = "{{database-name}}";
}
