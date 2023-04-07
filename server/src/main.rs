use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    name: String,
    port: u16,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let uri = req.uri();
    let query = uri.query().unwrap_or("");
    let params: Vec<&str> = query.split('&').collect();
    let mut a = None;
    let mut b = None;
    let mut op = None;
    for param in params {
        let pair: Vec<&str> = param.split('=').collect();
        if pair.len() != 2 {
            continue;
        }
        match pair[0] {
            "a" => a = Some(pair[1]),
            "b" => b = Some(pair[1]),
            "op" => op = Some(pair[1].chars().next().unwrap()),
            _ => (),
        }
    }
    if a.is_none() || b.is_none() || op.is_none() {
        return Ok(Response::builder()
            .status(400)
            .body("Bad Request".into())
            .unwrap());
    }
    let a = u16::from_str(a.unwrap()).unwrap();
    let b = u16::from_str(b.unwrap()).unwrap();
    let op = op.unwrap();
    let result = match op {
        '+' => a.wrapping_add(b),
        '-' => a.wrapping_sub(b),
        '*' => a.wrapping_mul(b),
        '/' => a.wrapping_div(b),
        _ => return Ok(Response::builder()
            .status(400)
            .body("Bad Request".into())
            .unwrap()),
    };
    Ok(Response::new(result.to_string().into()))
}

#[derive(StructOpt, Debug)]
#[structopt(name = "http-server")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    config: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let mut file = File::open(opt.config).expect("failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read config file");
    let config: ServerConfig = serde_json::from_str(&contents).expect("failed to parse config file");

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("{} server listening on http://{}", config.name, addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}