// Add the necessary imports
use reqwest;

// Define the main function
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Specify the URL you want to make a request to
    let url = "https://jsonplaceholder.typicode.com/todos/3";

    // Make the HTTP GET request
    let response = reqwest::get(url).await?;

    // Check if the request was successful (status code 2xx)
    if response.status().is_success() {
        // Print the response body as a string
        let body = response.text().await?;
        println!("Response body: {}", body);
    } else {
        // Print an error message if the request was not successful
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
