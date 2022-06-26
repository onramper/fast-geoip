use crate::utils::{binary_search, ip_string_to_number};
use moka::sync::Cache;
use once_cell::sync::Lazy;
use std::io::BufReader;
use std::path::Path;

static CACHE: Lazy<Cache<&str, Vec<u32>>> = Lazy::new(|| Cache::new(10_000));

static ROOT: &str = "../data";
static CACHE_ENABLED: bool = false;

#[derive(Debug)]
pub struct IpInfo {
    range: (u64, u64),
    country: String,
    region: String,
    eu: bool,
    timezone: String,
    city: String,
    ll: (f32, f32),
    metro: u32,
    area: u32,
}

#[derive(Clone)]
//struct FileFormat<'a> {
//index_file: u8,
//// TODO: Check types for ip_block_record and location_record
//ip_block_record: Vec<(u8, Option<u8>, u8, u8, u8)>,
//location_record: Vec<(&'a str, &'a str, &'a str, u8, &'a str, bool)>,
//}

struct FileFormat<'a>(
    u8,
    Vec<(u8, Option<u8>, u8, u8, u8)>,
    Vec<(&'a str, &'a str, &'a str, u8, &'a str, bool)>,
);

impl IpInfo {
    pub async fn lookup4(ipv4: &str) -> std::io::Result<Self> {
        let ip = ip_string_to_number(ipv4);

        let mut root_index: isize;

        let mut next_ip = ip_string_to_number("255.255.255.255".into());

        let file = FileFormat::read_file("index.json")
            .await
            .expect("Failed to read the file.");

        root_index = binary_search(file, ip, |x| x);

        if root_index == -1 {
            panic!("Ip not found in the database")
        }

        Ok(IpInfo {
            range: (0, 0),
            country: "IT".to_string(),
            region: "25".to_string(),
            eu: true,
            timezone: "Europe/Rome".to_string(),
            city: "Milan".to_string(),
            ll: (0.0, 0.0),
            metro: 0,
            area: 20,
        })
        //next_ip = Self::get_next_ip(file, index, current_next_ip, extract_key_fn)
    }

    //fn get_next_ip<F>(
    //list: Vec<FileFormat>,
    //index: usize,
    //current_next_ip: u8,
    //extract_key_fn: F,
    //) -> u8
    //where
    //F: FnOnce(FileFormat) -> u8,
    //{
    //if index < (list.len() - 1) {
    //extract_key_fn(list[index + 1])
    //} else {
    //current_next_ip
    //}
    //}
}

impl<'a> FileFormat<'a> {
    async fn read_file(file_name: &'static str) -> std::io::Result<Vec<u32>> {
        if CACHE_ENABLED && CACHE.get(&file_name).is_some() {
            return Ok(CACHE.get(&file_name).unwrap());
        }

        let file = async_fs::read(Path::new(ROOT).join(file_name))
            .await
            .expect("Failed to read the file. Filename: {file_name}");

        let json: Vec<u32> = serde_json::from_reader(BufReader::new(file.as_slice())).unwrap();

        if CACHE_ENABLED {
            CACHE.insert(file_name, json.clone())
        }

        Ok(json)
    }
}
