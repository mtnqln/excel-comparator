use rust_xlsxwriter::Workbook;

use crate::domain::{models, ports::WriteExcel};

#[derive(Clone)]
pub struct ExcelWriter;

impl ExcelWriter{
    pub fn new()->Self{
        Self
    }
}

impl WriteExcel for ExcelWriter{
    fn write_data(&self,path:&str,vec_line:Vec<crate::domain::models::Line>)->Result<(),crate::domain::models::ExcelWriterError> {
        let mut workbook = Workbook::new();
        // Add a worksheet to the workbook.
        let worksheet = workbook.add_worksheet();
        // Set the column width for clarity.
        worksheet.set_column_width(0, 22).map_err(|e|{models::ExcelWriterError(e.to_string())})?;
        // Pointers for the excel
        let mut row = 0;
        
        for line in vec_line{
            worksheet.write(row, 0, line.first_line).map_err(|e|{models::ExcelWriterError(e.to_string())})?;
            worksheet.write(row,1,line.second_line).map_err(|e|{models::ExcelWriterError(e.to_string())})?;
            row+=1;
        }

        workbook.save(path).map_err(|e| {models::ExcelWriterError(e.to_string())})?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::domain::models::Line;

    use super::*;

    #[test]
    fn test_write_data() {
        // Temp file path
        let file = tempfile::NamedTempFile::new().unwrap();
        let test_file_path = file.path();
        
        let line_1:Line = Line::new("first_line".to_string(), "second_line".to_string());
        let line_2:Line = Line::new("first_line".to_string(), "second_line".to_string());
        
        let vec_line = vec![line_1,line_2];

        let writer = ExcelWriter::new();

        println!("{:?}",test_file_path);
        let result = writer.write_data(test_file_path.to_str().unwrap(), vec_line);
        assert!(result.is_ok(),"Writing file failed");

        assert!(Path::new(test_file_path).exists(),"Excel file wasn't created");        

    }
}