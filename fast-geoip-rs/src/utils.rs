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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_ip_coercion_from_string_to_number() {
        assert_eq!(ip_string_to_number("255.255.255.255"), 4294967295);
        assert_eq!(ip_string_to_number("81.22.36.183"), 1360405687);
    }
}
