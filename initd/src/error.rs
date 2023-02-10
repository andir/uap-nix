#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to create directory {directory}, errno: {errno}, message: {message}")]
    FailedToCreateDirectory {
        message: &'static str,
        directory: String,
        errno: nc::Errno,
	error_message: &'static str,
    },

    #[error("Directory already exists. {message}: {directory}")]
    DirectoryAlreadyExistsAsFile {
        message: &'static str,
        directory: String,
    },

    #[error("Failed to mount at {directory}, errno: {error}")]
    FailedToMount { directory: String, error: nc::Errno },

    #[error("All children died")]
    AllChildrenDied,

    #[error("Failed to convert Rust string to FFI compatible CString")]
    FailedToConvertStringToCString,

    #[error("errno {errno}: {message}")]
    Errno {
	errno: nc::Errno,
	message: &'static str,
    },

    #[error("errno: {errno_message} ({errno}) while {message}")]
    ErrnoWithMessage {
	errno: nc::Errno,
	errno_message: &'static str,
	message: &'static str,
    },

    #[error("Failed to convert readlink result to rust string: {error:?}")]
    FailedToConvertReadlinkResultToString { error: std::string::FromUtf8Error },

    #[error("task died")]
    TaskDied,

    #[error("Failed to create a pipe(2)")]
    PipeCreationFailed,

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error)
}


impl Error {
    pub fn from_errno(value: nc::Errno) -> Self {
	Self::Errno {
	    errno: value,
	    message: nc::strerror(value),
	}
    }

    pub fn from_errno_with_message(errno: nc::Errno, message: &'static str) -> Self {
	Self::ErrnoWithMessage { errno, errno_message: nc::strerror(errno), message, }
    }
}
