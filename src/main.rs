use rust_decimal::Decimal;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=5433 user=user password=password dbname=test",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT price FROM prices ORDER BY price", &[])
        .await?;

    let prices = rows
        .iter()
        .map(|r| r.get::<_, Decimal>(0))
        .collect::<Vec<_>>();

    let price_1 = prices[0];
    let price_2 = prices[1];

    let sum = price_1 + price_2;

    println!("Price 1: {price_1}");
    println!("Price 2: {price_2}");
    println!("Incorrect summation: {}", sum);

    // Get the `flags` field of `price_2`.
    // Can also check this in a debugger.
    let flags = unsafe {
        let ptr = &price_2 as *const Decimal;
        let flags_ptr = ptr.offset(0) as *const u32;
        *flags_ptr
    };

    println!("Flags in hex for Price 2: {flags:x}");
    println!("Flags in binary for Price 2: {flags:b}");

    let scale = (flags >> 16) & 0xff;
    println!("Scale for Price 2 (valid values are 0-28): {scale}, binary: {scale:b}");

    Ok(())
}
