use crate::utils::{file_binary_search, ip_string_to_number, item_binary_search};
use config::{Config, File, FileFormat};
use futures::AsyncReadExt;
use moka::sync::Cache;
use once_cell::sync::Lazy;
use serde::{de, Deserialize};
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader, SeekFrom};
use std::path::Path;
use std::ptr::read;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIGURATION: HashMap<String, usize> = {
        let config_builder = Config::builder().add_source(File::new("params", FileFormat::Json));

        let config = config_builder
            .build()
            .expect("Failed to load the internal library configuration");

        config.try_deserialize::<HashMap<String, usize>>().unwrap()
    };
}

//static CACHE: Lazy<Cache<&str, Vec<u32>>> = Lazy::new(|| Cache::new(10_000));

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct IpBlockRecord(pub u32, pub Option<u32>, pub f32, pub f32, pub u16);

static ROOT: &str = "../data";
static CACHE_ENABLED: bool = false;

#[derive(Debug, PartialEq)]
pub struct IpInfo {
    pub range: (u32, u32),
    pub country: String,
    pub region: String,
    // TODO: check if possible transform to a boolean
    pub eu: String, // "1" | "0"
    pub timezone: String,
    pub city: String,
    pub ll: (f32, f32),
    pub metro: u32,
    pub area: u16,
}

struct Params {
    number_nodes_per_midindex: u8,
    location_record_size: u8,
}

impl IpInfo {
    pub async fn lookup4(ipv4: &str) -> std::io::Result<Self> {
        let ip = ip_string_to_number(ipv4);

        let mut next_ip = ip_string_to_number("255.255.255.255".into());

        match read_file::<u32>("index.json").await {
            Ok(file) => {
                let root_index: isize = file_binary_search(&file, ip);

                if root_index == -1 {
                    panic!("Ip not found in the database")
                }

                next_ip = Self::get_next_ip_from_u32(&file, root_index, next_ip);

                match read_file::<u32>(&format!("i{}.json", &root_index)).await {
                    Ok(file) => {
                        let index = file_binary_search(&file, ip)
                            + root_index
                                * CONFIGURATION
                                    .get("NUMBER_NODES_PER_MIDINDEX")
                                    .expect("Failed to fetch internal library parameters")
                                    .clone() as isize;

                        next_ip = Self::get_next_ip_from_u32(&file, index, next_ip);

                        match read_file::<IpBlockRecord>(&format!("{index}.json")).await {
                            Ok(file) => {
                                let index = item_binary_search(&file, ip);

                                let ip_data = file[index as usize];

                                if ip_data.1 == None {
                                    panic!("1: IP doesn't any region nor country associated");
                                };

                                next_ip = Self::get_next_ip_from_list(&file, index, next_ip);

                                match read_location_record(ip_data.1.unwrap()).await {
                                    Ok(data) => Ok(IpInfo {
                                        range: (ip_data.0, next_ip),
                                        country: data.0,
                                        region: data.1,
                                        eu: data.5,
                                        timezone: data.4,
                                        city: data.2,
                                        ll: (ip_data.2, ip_data.3),
                                        metro: data.3,
                                        area: ip_data.4,
                                    }),
                                    _ => panic!("2: IP doesn't any region nor country associated"),
                                }
                            }
                            _ => panic!("IP doesn't any region nor country associated"),
                        }
                    }
                    _ => panic!("Failed to read the next index file."),
                }
            }
            _ => panic!("Failed to read the internal index file"),
        }
    }

    fn get_next_ip_from_u32(list: &Vec<u32>, index: isize, current_next_ip: u32) -> u32 {
        if index < (list.len() - 1) as isize {
            list[(index as usize) + 1]
        } else {
            current_next_ip
        }
    }
    fn get_next_ip_from_list(list: &Vec<IpBlockRecord>, index: isize, current_next_ip: u32) -> u32 {
        if index < (list.len() - 1) as isize {
            list[(index as usize) + 1].0
        } else {
            current_next_ip
        }
    }
}

// TODO: check number type
#[derive(Deserialize, Debug)]
struct LocationRecord(String, String, String, u32, String, String);

async fn read_location_record(index: u32) -> io::Result<LocationRecord> {
    let location_record_size = CONFIGURATION
        .get("LOCATION_RECORD_SIZE")
        .expect("Failed to read the params internal file");

    read_file_chunk::<LocationRecord>(
        "locations.json",
        ((index as usize) * location_record_size + 1) as u64,
        location_record_size - 1,
    )
    .await
}

async fn read_file_chunk<'b, T: serde::de::DeserializeOwned>(
    file_name: &str,
    offset: u64,
    lenght: usize,
) -> io::Result<T> {
    // TODO: read file async
    let mut file =
        std::fs::File::open(Path::new(ROOT).join(file_name)).expect("Location file not found");

    let mut reader = vec![0; lenght];

    file.seek(SeekFrom::Start(offset))
        .expect("Failed to seek test start of the search buffer.");

    file.read_exact(&mut reader)
        .expect("Failed to read the searched buffer.");

    let buffer = BufReader::new(reader.as_slice());

    // TODO: Close opened file?
    let result: T = serde_json::from_reader(buffer)
        .expect("Unable to deserialize the locations.json chunk file.");

    Ok(result)
}

async fn read_file<'a, T: serde::de::DeserializeOwned>(file_name: &str) -> std::io::Result<Vec<T>> {
    //if CACHE_ENABLED && CACHE.get(&file_name).is_some() {
    //return Ok(CACHE.get(&file_name).unwrap());
    //}

    let file = async_fs::read(Path::new(ROOT).join(file_name))
        .await
        .expect("Failed to read the file. Filename: {file_name}");

    let buffer = BufReader::new(file.as_slice());

    // TODO: maybe ::from_string() is more fast
    let json: Vec<T> = serde_json::from_reader(buffer).unwrap();

    //if CACHE_ENABLED {
    //CACHE.insert(file_name, json.clone())
    //}

    Ok(json)
}
