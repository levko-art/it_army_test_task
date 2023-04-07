use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    a: u16,
    b: u16,
    op: String,
    #[structopt(short = "s", long = "server")]
    server_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let url = format!(
        "{}?a={}&b={}&op={}",
        args.server_url, args.a, args.b, args.op
    );
    let resp = reqwest::get(&url).await?;

    let body = resp.text().await?;
    println!("{}", body);

    Ok(())
}