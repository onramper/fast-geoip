use fast_geoip_rs::IpInfo;

#[tokio::test]
async fn check_lookup4_result() {
    let geoip = IpInfo::lookup4("81.22.36.183").await.unwrap();

    assert_eq!(
        geoip,
        IpInfo {
            range: (1360405504, 1360405760),
            country: "IT".to_string(),
            region: "25".to_string(),
            eu: "1".to_string(),
            timezone: "Europe/Rome".to_string(),
            city: "Milan".to_string(),
            ll: (45.4722, 9.1922),
            metro: 0,
            area: 20
        }
    );
}
