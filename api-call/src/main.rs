use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct User{
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error>{
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
    owner = "rust-lang-nursery",
    repo = "rust-cookbook"
    );
    println!("{}",request_url);
    let clinet = reqwest::Client::new();
    let response = clinet
    .get(&request_url)
    .header(USER_AGENT, "rust-web-api-clinet demo")
    .send()
    .await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    
    Ok(())

}