use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("File does not exist: {0}")]
    FileNotFound(String),

    #[error("Invalid file extension: {0}. Expected one of: .fasta, .fa, .fsa, .fna")]
    InvalidFileExtension(String),

    #[error("Failed to read fasta file: {0}")]
    FastaReadError(String),

    #[error("Progress bar template error: {0}")]
    IndicatifError(#[from] indicatif::style::TemplateError),
}
