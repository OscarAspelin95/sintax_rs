use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Fasta Does Not Exist")]
    FastaPathError,

    #[error("Fasta Could Not Be Read")]
    FastaReadError,
}
