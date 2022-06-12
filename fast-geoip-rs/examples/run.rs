use fast_geoip_rs::get;

fn main() {
    let data = get("this", "81.10.20.40").unwrap();
}
