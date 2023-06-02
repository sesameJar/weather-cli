use dotenv;
use reqwest::blocking::get;
use clap::Parser;

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
fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().unwrap();

    let mut api_key = None;

    for (key, value) in std::env::vars() {
        if key != "APIKEY" {
            continue;
        }
        api_key = Some(value);
    }

    if api_key.is_none() {
        panic!("Wrong API Key");
    }
    let api_key = api_key.unwrap();

    let args = Args::parse();

    let method = match args.days
    {
        0 => "weather",
        _ => "forecast",     
    };

    let cnt = args.days * 8;

    let url = format!("https://api.openweathermap.org/data/2.5/{method}?lat={LAT}&lon={LON}&appid={api_key}&units=metric&cnt={cnt}");
    println!("{}",url );

    let body: serde_json::Value = get(url)?.json()?;

    let temp = body["main"]["temp"].as_f64().unwrap_or_default();
    println!("Temprature is : {:?} C", temp);

    Ok(())
}
