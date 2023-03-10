use diesel::result::Error;
use tonic::Status;
use bcrypt::BcryptError;

pub fn sql_err_to_grpc_error(error: Error) -> Status {
    match error {
        Error::NotFound => Status::not_found("not found".to_string()),
        _ => Status::internal(error.to_string()),
    }
}

pub fn bcrypt_err_to_grpc_error(error: BcryptError) -> Status {
    match error {
        BcryptError::InvalidHash(_) => Status::permission_denied("invalid password".to_string()),
        _ => Status::internal(error.to_string()),
    }
}
