use calamine::{open_workbook, Xlsx, Reader};
use crate::domain::ports::GetExcel;
use crate::domain::models::{self, Line};
use std::env;

#[derive(Clone)]
pub struct ExcelReader;

impl ExcelReader{
    pub fn new()-> Self{
        Self
    }
}

impl GetExcel for ExcelReader{
    fn get_data(&self,input_file:&str)->Result<Vec<crate::domain::models::Line>,models::ExcelReaderError> {
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().expect("Impossible to obtain .exe file");
        let path = format!("{}/{}", exe_dir.to_str().unwrap(),input_file);
        println!("Path found : {}",path);
        let mut line_vector:Vec<models::Line> = Vec::new();

        let mut excel:Xlsx<_> = open_workbook(path).map_err(|err: calamine::XlsxError| {models::ExcelReaderError(err.to_string())})?;
        
        if let Ok(r) = excel.worksheet_range("Sheet1") {
            for row in r.rows(){
                line_vector.push(Line::new(row[0].to_string(),row[1].to_string()));
            }
        }

        if let Ok(r) = excel.worksheet_range("Feuil1") {
            for row in r.rows(){
                line_vector.push(Line::new(row[0].to_string(),row[1].to_string()));
            }
        }
        Ok(line_vector)
    }
        
}

