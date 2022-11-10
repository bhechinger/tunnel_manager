use sqlx::Error;
use tonic::Status;

pub fn sql_err_to_grpc_error(error: Error) -> Status {
    match error {
        Error::RowNotFound => Status::not_found(error.to_string()),
        Error::TypeNotFound { type_name: _ } => Status::not_found(error.to_string()),
        Error::ColumnIndexOutOfBounds { .. } => Status::out_of_range(error.to_string()),
        Error::ColumnNotFound(_) => Status::not_found(error.to_string()),

        Error::Configuration(_) => Status::internal(error.to_string()),
        Error::Database(_) => Status::internal(error.to_string()),
        Error::Io(_) => Status::internal(error.to_string()),
        Error::Tls(_) => Status::internal(error.to_string()),
        Error::Protocol(_) => Status::internal(error.to_string()),
        Error::ColumnDecode { .. } => Status::internal(error.to_string()),
        Error::Decode(_) => Status::internal(error.to_string()),
        Error::PoolTimedOut => Status::internal(error.to_string()),
        Error::PoolClosed => Status::internal(error.to_string()),
        Error::WorkerCrashed => Status::internal(error.to_string()),
        Error::Migrate(_) => Status::internal(error.to_string()),
        _ => Status::internal(error.to_string()),
    }
}
