use anyhow::Result;
use super::models::{ApiAnswer,Line};
use super::ports::{ApiLlm, GetExcel, WriteExcel};

#[derive(Debug,Clone)]
pub struct ServiceLineComparer<G,A,W>
where 
    G:GetExcel,
    A:ApiLlm,
    W:WriteExcel
{
    input_excel:G,
    api:A,
    output_excel_same:W,
    output_excel_diff:W,
}

impl<G,A,W> ServiceLineComparer<G,A,W>
where 
    G:GetExcel,
    A:ApiLlm,
    W:WriteExcel
{
    pub fn new(input_excel:G,api:A,output_excel_same:W,output_excel_diff:W)->Self{
        Self { input_excel, api, output_excel_same,output_excel_diff }
    }

    pub async fn process(&self,input_file:&str)->Result<()>{

        println!("Starting the process");
        // Initializing the data
        let data = self.input_excel.get_data(input_file)?;
        let data = self.api.send_api(data).await?;
        let mut api_result_same_vec:Vec<Line> = Vec::new();
        let mut api_result_diff_vec:Vec<Line> = Vec::new();
        // Iterating through the lines
        for line in data.into_iter().enumerate(){
            match line.1 {
                ApiAnswer::Same(line)=>api_result_same_vec.push(line),
                ApiAnswer::Diff(line)=>api_result_diff_vec.push(line)
            }
        }
        
        println!("Writing processed data");
        self.output_excel_same.write_data(format!("{}{}","same_",input_file).as_str(),api_result_same_vec)?;
        self.output_excel_diff.write_data(format!("{}{}","diff_",input_file).as_str(),api_result_diff_vec)?;

        println!("Finished all processes");
        Ok(()) 
    }
}



