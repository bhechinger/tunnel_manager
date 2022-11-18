use diesel::result::Error;
use tonic::Status;

pub fn sql_err_to_grpc_error(error: Error) -> Status {
    match error {
        Error::RowNotFound => Status::not_found("not found".to_string()),
        Error::TypeNotFound { .. } => Status::not_found(error.to_string()),
        Error::ColumnIndexOutOfBounds { .. } => Status::out_of_range(error.to_string()),
        Error::ColumnNotFound(_) => Status::not_found(error.to_string()),
        _ => Status::internal(error.to_string()),
    }
}
