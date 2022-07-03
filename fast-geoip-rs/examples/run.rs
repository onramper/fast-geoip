use fast_geoip_rs::IpInfo;
use futures::executor::block_on;

fn main() {
    block_on(async_code());
}

async fn async_code() {
    let data = IpInfo::lookup4("81.22.36.183").await;

    //let data = IpInfo::lookup4("255.255.255.256").await;
    match data {
        Ok(val) => println!("{:?}", val),
        Err(e) => println!("I'd like to panic"),
    }
}
