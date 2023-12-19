use mongodb::{Client, options::ClientOptions};



pub async fn get_db() -> Result<Client, mongodb::error::Error> {
    let client_options = ClientOptions::parse("mongodb+srv://aman:aman@cluster0.suqkggz.mongodb.net/?retryWrites=true&w=majority").await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}




