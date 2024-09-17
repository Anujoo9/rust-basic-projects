n main() -> Result<(), Error>{
    // make you already created library DATABASE
    let mut client = Client::connect("postgres://postgres:mysecretpassword@localhost:5432/library", NoTls)?;
    
    client.batch_execute("
    CREATE TABLE IF NOT EXISTS author(
    id  SERIAL PRIMARY KEY,
    name  VARCHAR NOT NULL,
    country VARCHAR NOT NULL
    )
    ")?;
    
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book(
        id  SERIAL PRIMARY KEY,
        title VARCHAR NOT NULL,
        
        author_id INTEGER NOT NULL REFERENCES author
        )
    ")?;

    Ok(())
}
