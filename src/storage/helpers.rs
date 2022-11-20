use diesel::result::Error;
use tonic::Status;

pub fn sql_err_to_grpc_error(error: Error) -> Status {
    match error {
        Error::NotFound => Status::not_found("not found".to_string()),
        _ => Status::internal(error.to_string()),
    }
}
