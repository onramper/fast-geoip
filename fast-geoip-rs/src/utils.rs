use std::iter::{ExactSizeIterator, Iterator};

pub fn bynary_search<T, F>(
    list: impl Iterator + ExactSizeIterator,
    item: usize,
    extract_key_fn: F,
) -> isize
where
    F: FnOnce(T) -> usize,
{
    let mut low: usize = 0;
    let mut hight: usize = list.len() - 1;

    loop {
        let index: usize = (hight - low) / 2 + low;

        if item < extract_key_fn(list[index]) {
            if index == hight && index == low {
                return -1;
            } else if index == hight {
                hight = low;
            } else {
                hight = index;
            }
        } else if item >= extract_key_fn(list[index]) && index == (list.len() - 1)
            || item < extract_key_fn(list[index + 1])
        {
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
            acc + val * 256_u32.pow(3 - index as u32)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_ip_coercion_from_string_to_number() {
        assert_eq!(ip_string_to_number("255.255.255.255"), 4294967295);
    }
}
