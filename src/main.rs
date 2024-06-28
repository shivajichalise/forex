use clap::Parser;
use colored::Colorize;
use forex::Args;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct CurrencyData {
    iso3: String,
    name: String,
    unit: u32,
    buy: String,
    sell: String,
    date: String,
    published_on: String,
    modified_on: String,
}

struct Rate {
    buy: f64,
    sell: f64,
}

const URL: &str = "https://www.nrb.org.np/api/forex/v1/app-rate";

fn get_rate(
    rates_map: HashMap<String, CurrencyData>,
    currency: &String,
    amount: u32,
) -> Result<Rate, ()> {
    match rates_map.get(&currency.to_lowercase()) {
        Some(rate) => {
            let buy = (rate.buy.parse::<f64>().unwrap() / f64::from(rate.unit)) * f64::from(amount);
            let sell =
                (rate.sell.parse::<f64>().unwrap() / f64::from(rate.unit)) * f64::from(amount);
            Ok(Rate { buy, sell })
        }
        None => {
            println!("Rates for currency {} not found.", currency);
            Err(())
        }
    }
}

fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let currency = args.currency;
    let amount = args.amount;

    let client: Client = Client::new();
    let mut rates_map: HashMap<String, CurrencyData> = HashMap::new();

    let res = client.get(URL).send();

    match res {
        Ok(res) => {
            rates_map = res
                .json::<Vec<CurrencyData>>()?
                .into_iter()
                .map(|rate| (rate.iso3.to_lowercase(), rate))
                .collect();
        }
        Err(err) => println!("error = {err}"),
    }

    match get_rate(rates_map, &currency, amount) {
        Ok(rate) => {
            let buy = rate.buy;
            let sell = rate.sell;

            let mut buy_string = buy.to_string().green();
            let mut sell_string = sell.to_string().red();

            if buy > sell {
                buy_string = buy.to_string().red();
                sell_string = sell.to_string().green();
            }

            println!(
                "{} {} -> {}",
                amount.to_string().normal().bold(),
                &currency.to_uppercase().normal().bold(),
                "NPR".normal().bold()
            );
            println!("{} -> {}", "Buy".blue(), buy_string);
            println!("{} -> {}", "Sell".blue(), sell_string);
        }
        Err(_) => {}
    }

    Ok(())
}
