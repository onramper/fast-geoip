use crate::binary_search;
use futures::{future::Future, io};
use moka::sync::Cache;
use once_cell::sync::Lazy;
use std::io::Read;
use std::mem;
use std::net::Ipv4Addr;
use std::path::Path;
use std::slice::from_raw_parts_mut;

static CACHE: Lazy<Cache<&str, FileFormat>> = Lazy::new(|| Cache::new(10_000));

static ROOT: &str = "../data";
static CACHE_ENABLED: bool = false;

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

struct FileFormat<'a>(u8, Vec<(u8, Option<u8>,u8,u8,u8)>, Vec<(&'a str,&'a str, &'a str, u8, &'a str, bool)>);

impl IpInfo {
    async fn lookup4(ipv4: &str) -> io::Result<Self> {
        let ip: Ipv4Addr = ipv4.parse().expect("Failed to parse the ip. Value: {ipv4}");

        let mut root_index: u8;

        let mut next_ip: u8 = Ipv4Addr::BROADCAST.;

        let file = FileFormat::read_file("index.json")
            .await
            .expect("Failed to read the file.");

        root_index = binary_search(file, ip, |x| x);

        if root_index == -1 {
            panic!("Ip not found in the database");
        }

        next_ip = Self::get_next_ip(file, index, current_next_ip, extract_key_fn)
    }

    fn get_next_ip<F>(
        list: Vec<FileFormat>,
        index: usize,
        current_next_ip: u8,
        extract_key_fn: F,
    ) -> u8
    where
        F: FnOnce(FileFormat) -> u8,
    {
        if index < (list.len() - 1) {
            extract_key_fn(list[index + 1])
        } else {
            current_next_ip
        }
    }
}

impl FileFormat {
    async fn read_file(file_name: &str) -> io::Result<Self> {
        if CACHE_ENABLED && CACHE.get(&file_name).is_some() {
            return Ok(CACHE.get(&file_name).unwrap());
        }

        let file = tokio::fs::read(Path::new(ROOT).join(file_name))
            .await
            .expect("Failed to read the file. Filename: {file_name}");

        let file = unsafe {
            Self::struct_from_file(&file[..]).expect("Failed to parse the file to structured data.")
        };

        if CACHE_ENABLED {
            CACHE.insert(file_name, file)
        }

        Ok(file)
    }

    unsafe fn struct_from_file(src: impl Read) -> io::Result<FileFormat<'static>> {
        let mut buffer = mem::MaybeUninit::uninit();
        let buffer_slice =
            from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, mem::size_of::<FileFormat>());

        src.read_exact(buffer_slice)?;
        Ok(buffer.assume_init())
    }
}
