use std::io;
// Imports architecture hexagonale inbound outbound and domain
use excel_comparator::inbound::excel_reader::ExcelReader;         // To read excel files
use excel_comparator::outbound::excel_writer::ExcelWriter;         // To write excel files
use excel_comparator::outbound::mistral_client::mistral_api::LlmAdapter;             // Adapter for API calls
use excel_comparator::domain::service::ServiceLineComparer;      // Service 

#[tokio::main]
async fn main()->anyhow::Result<(),anyhow::Error>{
    // Reading API KEY
    let mut api_key = String::new();
    println!("Enter your API KEY : ");
    io::stdin()
    .read_line(&mut api_key)
    .expect("Error reading API KEY");

    let mistral_api = LlmAdapter::new(api_key.trim().to_string());
    let excel_reader = ExcelReader::new();
    let output_excel_same = ExcelWriter::new();
    let output_excel_diff = ExcelWriter::new();

    let line_comparator = ServiceLineComparer::new(excel_reader,mistral_api, output_excel_same, output_excel_diff);
    
    // Reading input file name
    println!("Enter your file name : ");
    let mut input_file = String::new();
    io::stdin()
    .read_line(&mut input_file)
    .expect("Error reading input file name");

    line_comparator.process(input_file.trim()).await?;

    Ok(())
}

