use std::cmp::{Ordering, PartialOrd};

pub fn run() {
    let data = [1, 2, 5, 7, 13, 17, 20, 22, 35, 66, 78, 80, 90, 99];
    let item = 66;

    let index = search(&data, item);

    println!("The index of {} is {:?}", item, index);
}

fn search<T: PartialOrd>(data: &[T], item: T) -> Option<usize> {
    let mid = data.len() / 2;

    match data.get(mid) {
        None => None,
        Some(val) => {
            if *val == item {
                Some(mid)
            } else if *val > item {
                let subdata = &data[..mid];
                search(subdata, item)
            } else {
                let subdata = &data[(mid + 1)..];
                search(subdata, item).map(|pos| pos + mid + 1)
            }
        },
    }
}