use crate::domain::models::Line;
use super::models::{ApiAnswer, ApiRequestError, ExcelReaderError, ExcelWriterError};

pub trait GetExcel:Clone + Send + Sync{
    fn get_data(&self,input_file:&str)->Result<Vec<Line>,ExcelReaderError>;
}

pub trait ApiLlm: Clone + Send + Sync {
    fn send_api(&self, vec_line:Vec<Line>) -> impl std::future::Future<Output = Result<Vec<ApiAnswer>,ApiRequestError>> + Send;
}

pub trait WriteExcel:Clone + Send + Sync{
    fn write_data(&self,path:&str,vec_line:Vec<Line>)->Result<(),ExcelWriterError>;
}