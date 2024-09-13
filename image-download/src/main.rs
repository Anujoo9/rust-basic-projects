use error_chain::error_chain;
use std::io::Write;
use std::fs::{File, create_dir_all};
use std::path::PathBuf;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{

        let programming_dir = PathBuf::from("/Users/anujyadav/Anuj/programming");
        let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
        let response = reqwest::get(target).await?;

        let mut dest = {
            let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name | if name.is_empty(){None} else { Some(name)})
            .unwrap_or("tmp.bin");

        println!("File to download: {}", fname);
        let mut file_path = PathBuf::from(&programming_dir);
        file_path.push(fname);
        println!("will be located under: {}", file_path.display());

        File::create(file_path)?
        };
        let content = response.bytes().await?;
        dest.write_all(&content)?;
        Ok(())
}