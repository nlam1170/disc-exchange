#[path = "exchange.rs"] mod exchange;

pub async fn get_mex_cycle() -> String {
    let mex = exchange::Bitmex;
    let hub = exchange::Huboi;
    
    let (mex_funding, mex_oi, hub_funding, hub_oi) = tokio::join!(mex.get_funding_rate(), 
        mex.get_oi(),
        hub.get_funding_rate(),
        hub.get_oi(),
    );
    format!("__**Bitmex**__\nfunding:`{}`\noi:`{}`\n__**Huboi**__\nfunding:`{:.8}`\noi:`{}`",
         mex_funding.unwrap(), mex_oi.unwrap(), hub_funding.unwrap(), hub_oi.unwrap())
}

pub async fn get_ok_cycle() -> String {
    let ok = exchange::Okex;
    let by = exchange::Okex;
    let nance = exchange::Binance;
    
    let (ok_usd_f, ok_usd_oi, ok_usdt_f, ok_usdt_oi, binance_f, binance_oi, bybit_usd_f, bybit_usd_oi, bybit_usdt_f,  
        bybit_usdt_oi) = tokio::join!(ok.get_usd_funding(), ok.get_usd_oi(), ok.get_usdt_funding(),
        ok.get_usd_oi(),
        nance.get_funding_rate(),
        nance.get_oi(),
        by.get_usd_funding(),
        by.get_usd_oi(),
        by.get_usdt_funding(),
        by.get_usdt_oi(),
    );
    format!("__**Okex USD**__\nfunding`{}`\noi:`{}`\n__**Okex USDT**__\nfunding:`{}`\noi:`{}`\n__**Binance**__\nfunding`{}`\noi:`{}`\n__**Bybit USD**__\nfunding:`{}`\noi:`{}`\n__**Bybit USDT**__\nfunding:`{}`\noi:`{}`", 
    ok_usd_f.unwrap(), ok_usd_oi.unwrap(), ok_usdt_f.unwrap(), ok_usdt_oi.unwrap(), binance_f.unwrap(), binance_oi.unwrap(),
    bybit_usd_f.unwrap(), bybit_usd_oi.unwrap(), bybit_usdt_f.unwrap(), bybit_usdt_oi.unwrap())
} 
