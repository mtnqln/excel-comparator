use crate::domain::models::{self, ApiAnswer, ApiRequestError};
use reqwest::Client;
use serde_json::json;
use crate::domain::ports::ApiLlm;
use super::http_structure;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct LlmAdapter{api_key:String}

impl LlmAdapter {
    pub fn new(api_key:String) -> Self{
        Self{api_key}
    }
}

impl ApiLlm for LlmAdapter{
    async fn send_api(&self, vec_line:Vec<models::Line>) -> Result<Vec<ApiAnswer>, ApiRequestError> {
        // Mistral API url
        let url = "https://api.mistral.ai/v1/chat/completions";
        
        // Reqwest Client 
        let client = Client::new();

        let size = vec_line.len();
        
        // Initializing return vector
        let mut result_vec = Vec::with_capacity(size);

        // Iterating through the vector containing the sentences
        for line in vec_line.iter().enumerate(){
            // Bottleneck !!! (mistral API is max 1 rps and putting 1 sec sleep doesnt seem to work)
            sleep(Duration::from_secs(2)).await;
            // Bottleneck !!!
            let value = json!({
                "model": "open-mistral-nemo",
                "messages": [{"role": "user", "content": format!("Classify if the two french and english following sentences have the same meaning or not.
                Do not explain your choice, only answer with Yes or No. Example : 'I ate an apple' 'J'ai mangé une pomme', your answer : Yes.
                First sentence : {}. Second sentence : {}",line.1.first_line,line.1.second_line)}]});
            
            let response = client.post(url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}",self.api_key))
            .json(&value)
            .send()
            .await.map_err(|err|{models::ApiRequestError(err.to_string())})?;
            
            // Could have also used the .json from reqwest
            let response_text = response.text().await.map_err(|e| ApiRequestError(e.to_string()))?;
            let text: Result<http_structure::ChatResponse, ApiRequestError> = match serde_json::from_str(&response_text) {
                Ok(text) => Ok(text),
                Err(e) => {
                    println!("Response error: {}", response_text);
                    Err(ApiRequestError(e.to_string()))
                }
            };
            
            let clean_text:&String = &text?.choices[0].message.content;
            println!("Response : {}",clean_text);
            match clean_text.to_lowercase().as_str() {
                "yes" | "yes." => result_vec.push(ApiAnswer::Same(line.1.clone())),
                "no" | "no." => result_vec.push(ApiAnswer::Diff(line.1.clone())),
                _=>panic!("Error message LLM - TOFIX {}",clean_text),
            }
            eprintln!("Processed {} / {}",line.0+1,size);

            } 
        println!("{:?}",result_vec);
        Ok(result_vec)
        
    }
}
