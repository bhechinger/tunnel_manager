use sqlx::Error;
use tonic::Status;

pub fn sql_err_to_grpc_error(error: Error) -> Status {
    match error {
        Error::Configuration(_) => Status::internal(error.to_string()),
        _ => Status::unknown(error.to_string()),
    }
}
