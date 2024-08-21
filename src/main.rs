use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgres://postgres:postgres@localhost:5433/library", NoTls)?;

    let creation_result = create_tables(&mut client);

    match creation_result {
        Ok(()) => println!("Tables Created"),
        Err(e) => println!("{:?}", e),
    }

    let insert_result = add_author(&mut client, "Matt", "U.S.A");

    match insert_result {
        Ok(()) => println!("Data inserted into author table."),
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}

fn create_tables(client: &mut Client) -> Result<(), Error> {
    client.batch_execute(
        "
    CREATE TABLE IF NOT EXISTS author (
        id      SERIAL PRIMARY KEY,
        name    VARCHAR NOT NULL,
        country VARCHAR NOT NULL
    )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book (
            id          SERIAL PRIMARY KEY,
            title       VARCHAR NOT NULL,
            author_id   INTEGER NOT NULL REFERENCES author
        )
    ",
    )?;
    Ok(())
}

fn add_author(client: &mut Client, name: &str, country: &str) -> Result<(), Error> {
    client.execute(
        "INSERT INTO author (name, country) VALUES ($1, $2)",
        &[&name, &country],
    )?;

    Ok(())
}
