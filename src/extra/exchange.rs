use reqwest;
use std::error::Error;
use serde_json::Value;
use num_format::{Buffer, Locale};


async fn make_request(url: &str) -> Result<Value, Box<dyn Error>> {
    let resp = reqwest::get(url).await?.json().await?;
    Ok(resp)
}

fn add_commas<T>(num: T) -> String 
    where T: num_format::ToFormattedStr{
    let mut buf = Buffer::default();
    buf.write_formatted(&num, &Locale::en);
    let s = buf.as_str();
    s.to_string()
}

async fn get_usd_price() -> Result<i64, Box<dyn Error>> {
    let url = "https://www.okex.com/api/swap/v3/instruments/BTC-USD-SWAP/index";
    let resp = make_request(url).await?;
    let price = &resp["index"];
    let price: f64 = price.as_str().unwrap().parse().unwrap();
    Ok(price as i64) 
}

async fn get_usdt_price() -> Result<i64, Box<dyn Error>> {
    let url = "https://www.okex.com/api/swap/v3/instruments/BTC-USDT-SWAP/index";
    let resp = make_request(url).await?;
    let price = &resp["index"];
    let price: f64 = price.as_str().unwrap().parse().unwrap();
    Ok(price as i64) 
}

pub struct Bitmex;
pub struct Huboi;
pub struct Okex;
pub struct Binance;
pub struct Bybit;


impl Bitmex {
    pub async fn get_funding_rate(&self) -> Result<f64, Box<dyn Error>> {
        let url = "https://www.bitmex.com/api/v1/funding?symbol=XBTUSD&count=1&reverse=true";
        let resp = make_request(&url).await?;
        let funding_rate = &resp[0]["fundingRate"];
        Ok(funding_rate.as_f64().unwrap())
    }

    pub async fn get_oi(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://www.bitmex.com/api/v1/instrument?symbol=XBTUSD&count=1&reverse=true";
        let resp = make_request(&url).await?;
        let oi = &resp[0]["openInterest"];
        Ok(add_commas(oi.as_i64().unwrap()))
    }
}

impl Huboi {
    pub async fn get_funding_rate(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://api.btcgateway.pro/swap-api/v1/swap_funding_rate?&contract_code=BTC-USD";
        let resp = make_request(&url).await?;
        let funding_rate = &resp["data"]["funding_rate"];
        Ok(funding_rate.as_str().unwrap().parse().unwrap())
    }

    pub async fn get_oi(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://api.btcgateway.pro/swap-api/v1/swap_open_interest?contract_code=BTC-USD";
        let resp = make_request(&url).await?;
        let oi = &resp["data"][0]["volume"];
        Ok(add_commas(oi.as_f64().unwrap() as i64 * 100))
    }
}

impl Okex {
    pub async fn get_usd_funding(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://www.okex.com/api/swap/v3/instruments/BTC-USD-SWAP/funding_time";
        let resp = make_request(url).await?;
        let funding = &resp["funding_rate"];
        Ok(funding.as_str().unwrap().parse().unwrap())
    }

    pub async fn get_usd_oi(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://www.okex.com/api/swap/v3/instruments/BTC-USD-SWAP/open_interest";
        let resp = make_request(url).await?;
        let oi = &resp["amount"];
        let oi: i64 = oi.as_str().unwrap().parse().unwrap();
        Ok(add_commas(oi*100))
    }

    pub async fn get_usdt_funding(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://www.okex.com/api/swap/v3/instruments/BTC-USDT-SWAP/funding_time";
        let resp = make_request(url).await?;
        let funding = &resp["funding_rate"];
        Ok(funding.as_str().unwrap().parse().unwrap()) 
    }

    pub async fn get_usdt_oi(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://www.okex.com/api/swap/v3/instruments/BTC-USDT-SWAP/open_interest";
        let resp = make_request(url).await?;
        let oi = &resp["amount"];
        let oi: i64 = oi.as_str().unwrap().parse().unwrap();
        Ok(add_commas(oi*100))
    }
}

impl Binance {
    pub async fn get_funding_rate(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://fapi.binance.com/fapi/v1/fundingRate?limit=1&symbol=BTCUSDT";
        let resp = make_request(url).await?;
        let funding = &resp[0]["fundingRate"];
        Ok(funding.as_str().unwrap().parse().unwrap())
    }
    
    pub async fn get_oi(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://fapi.binance.com/fapi/v1/openInterest?symbol=BTCUSDT";
        let resp = make_request(url).await?;
        let oi = &resp["openInterest"];
        let oi: f64 = oi.as_str().unwrap().parse().unwrap();
        let index = get_usdt_price().await?;
        Ok(add_commas(oi as i64 * index))
    }
}

impl Bybit {
    pub async fn get_usd_funding(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://api.bybit.com/v2/public/tickers?symbol=BTCUSD";
        let resp = make_request(url).await?;
        let funding = &resp["result"][0]["funding_rate"];
        Ok(funding.as_str().unwrap().parse().unwrap())
    }

    pub async fn get_usd_oi(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://api.bybit.com/v2/public/tickers?symbol=BTCUSD";
        let resp = make_request(url).await?;
        let oi = &resp["result"][0]["open_interest"];
        Ok(add_commas(oi.as_i64().unwrap()))

    }

    pub async fn get_usdt_funding(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://api.bybit.com/v2/public/tickers?symbol=BTCUSDT";
        let resp = make_request(url).await?;
        let funding = &resp["result"][0]["funding_rate"];
        Ok(funding.as_str().unwrap().parse().unwrap())
    }

    pub async fn get_usdt_oi(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://api.bybit.com/v2/public/tickers?symbol=BTCUSDT";
        let (resp, index) = tokio::join!(make_request(url), get_usd_price());
        let oi = &resp.unwrap()["result"][0]["open_interest"];
        let oi = oi.as_f64().unwrap() as i64;
        let index = get_usd_price().await?;
        Ok(add_commas(oi * index))
    }
}
