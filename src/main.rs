#[macro_use]
extern crate trackable;

use clap::Parser;
use rusturn::auth::AuthParams;
use trackable::error::MainError;
use fibers_global;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long, default_value_t = 3478)]
    port: u16,
    
    #[clap(long, default_value = "foo")]
    username: String,

    /// Password.
    #[clap(long, default_value = "bar")]
    password: String,
}

fn main() -> Result<(), MainError>{
    let args = Args::parse();
    println!("running turn server on port {}", args.port);
    let addr = track_any_err!(format!("0.0.0.0:{}", args.port).parse())?;
    
    let auth_params = track!(AuthParams::new(&args.username, &args.password))?;

    let turn_server = track!(fibers_global::execute(rusturn::server::UdpServer::start(
        addr,
        auth_params,
    )))?;
    
    track!(fibers_global::execute(turn_server))?;

    Ok(())
}