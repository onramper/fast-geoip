use crate::core::IpBlockRecord;

pub fn item_binary_search(list: &Vec<IpBlockRecord>, item: u32) -> isize {
    let mut low: usize = 0;
    let mut high: usize = list.len() - 1;

    loop {
        let index: usize = (((((high as f32) - (low as f32)) / 2.0) as f32).round() as usize) + low;
        if item < list[index].0 {
            if index == high && index == low {
                return -1;
            } else if index == high {
                high = low;
            } else {
                high = index;
            }
        } else if item >= list[index].0 && (index == (list.len() - 1) || item < list[index + 1].0) {
            return index as isize;
        } else {
            low = index;
        }
    }
}

pub fn file_binary_search(list: &Vec<u32>, item: u32) -> isize {
    let mut low: usize = 0;
    let mut high: usize = list.len() - 1;

    loop {
        let index: usize = (((((high as f32) - (low as f32)) / 2.0) as f32).round() as usize) + low;
        println!(
            "binay search index: {index}; low: {low}; high: {high}; list {:?}",
            list.len()
        );

        println!("item: {item}, list[i]]: {}", list[index]);
        if item < list[index] {
            if index == high && index == low {
                return -1;
            } else if index == high {
                high = low;
            } else {
                high = index;
            }
        } else if item >= list[index] && (index == (list.len() - 1) || item < list[index + 1]) {
            return index as isize;
        } else {
            low = index;
        }
    }
}

pub fn ip_string_to_number(ip: &str) -> u32 {
    ip.split(".")
        .map(|x| x.parse::<u32>().unwrap())
        .enumerate()
        .fold(0, |val, (index, acc)| {
            return val + acc * 256_u32.pow(3 - index as u32);
        })
}

mod tests {
    use super::*;
    #[test]
    fn correct_ip_coercion_from_string_to_number() {
        assert_eq!(ip_string_to_number("255.255.255.255"), 4294967295);
        assert_eq!(ip_string_to_number("81.22.36.183"), 1360405687);
    }

    #[test]
    fn check_bynary_search_on_item() {
        let vector = vec![
            IpBlockRecord(1360699392, Some(43376), 48.7987, 8.3229, 20),
            IpBlockRecord(1360699904, Some(43376), 48.7987, 8.3229, 20),
            IpBlockRecord(1360700160, Some(42076), 48.9751, 8.4456, 20),
            IpBlockRecord(1360700416, Some(43376), 48.7987, 8.3229, 20),
            IpBlockRecord(1360701440, Some(43376), 48.7987, 8.3229, 20),
            IpBlockRecord(1360703488, Some(13213), 55.3662, 86.0805, 1000),
            IpBlockRecord(1360707584, Some(71040), 43.1479, 12.1097, 500),
            IpBlockRecord(1360709632, Some(72833), 41.3253, 19.8184, 50),
            IpBlockRecord(1360710144, Some(9540), 41.0, 20.0, 50),
            IpBlockRecord(1360710400, Some(72835), 42.0653, 19.51, 1000),
            IpBlockRecord(1360710416, Some(72833), 41.3253, 19.8184, 1000),
            IpBlockRecord(1360710432, Some(72833), 41.3253, 19.8184, 1000),
            IpBlockRecord(1360710464, Some(72833), 41.3253, 19.8184, 1000),
            IpBlockRecord(1360710528, Some(72833), 41.3253, 19.8184, 1000),
            IpBlockRecord(1360710656, Some(72833), 41.3253, 19.8184, 100),
            IpBlockRecord(1360710912, Some(9540), 41.0, 20.0, 50),
        ];
        assert_eq!(item_binary_search(&vector, 4500023), -1);
        assert_eq!(item_binary_search(&vector, 1360710528), 13);
        assert_eq!(item_binary_search(&vector, 1360707584), 6);
    }

    #[test]
    fn check_bynary_search_on_file() {
        let vector = vec![
            1332005376, 1332827136, 1333555712, 1334476816, 1335182656, 1335975936, 1336536896,
            1337127680, 1337448704, 1338048512, 1338815744, 1339677824, 1340403200, 1341185024,
            1341930496, 1342611456, 1343376128, 1344739072, 1345859328, 1346285568, 1347159840,
            1348001280, 1348782336, 1349500160, 1350081536, 1350984192, 1351830592, 1352809984,
            1353281184, 1353624576, 1354406912, 1355303680, 1356148224, 1356901632, 1357451520,
            1358161152, 1358861474, 1359589632, 1360337408, 1361225728, 1362046976, 1362966528,
            1364451200, 1365398784, 1366104576, 1367713952, 1368631296, 1369121280, 1369521408,
            1370078984, 1371038400, 1372106752, 1372464896, 1373201920, 1373681152, 1374055168,
            1374455808,
        ];

        assert_eq!(file_binary_search(&vector, 888), -1);
        assert_eq!(file_binary_search(&vector, 1332005376), 0);
        assert_eq!(file_binary_search(&vector, 1367713952), 45);
    }
}
