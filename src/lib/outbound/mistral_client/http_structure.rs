use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct ChatResponse{
    pub choices:Vec<Message>
}
#[derive(Debug,Deserialize,Serialize)]

pub struct Message{
    pub message:Content
}
#[derive(Debug,Deserialize,Serialize)]
pub struct Content{
    pub content:String
}
