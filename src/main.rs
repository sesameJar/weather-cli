use clap::Parser;
use reqwest;

const LAT: f32 = -41.2;
const LON: f32 = 174.7;

#[derive(Parser)]
#[command(name="wet")]
#[command(about = "Weather CLI", long_about = None)]
struct Args {
    //num of days to forecast
    #[arg(short, default_value_t = 0)]
    days: u8,
}
fn main() {
    let args = Args::parse();
    let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
    println!("body = {}", body);
}
