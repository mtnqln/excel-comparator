use thiserror::Error;

#[derive(Debug,Clone)]
pub struct Line{
    pub first_line:String,
    pub second_line:String,
}

impl Line{
    pub fn new(first_line: String,second_line: String)-> Self {
        Self { first_line, second_line }
    }
}

#[derive(Debug,Clone)]
pub enum ApiAnswer{
    Same(Line),
    Diff(Line),
}

#[derive(Error,Debug,Clone)]
#[error("Error using the Mistral API : {0}")]
pub struct ApiRequestError(pub String);

#[derive(Error,Debug,Clone)]
#[error("Error reading Excel file : {0}")]
pub struct ExcelReaderError(pub String);

#[derive(Error,Debug,Clone)]
#[error("Error writing excel files : {0}")]
pub struct ExcelWriterError(pub String);

