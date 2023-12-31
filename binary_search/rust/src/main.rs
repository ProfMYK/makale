use std::fs;

fn binary_search(k: i32, items: &Vec<i32>) -> Option<i32> {
    let mut low: i32 = 0;
    let mut high: i32 = items.len() as i32 - 1;
 
    while low <= high {
        let middle = (((high + low) / 2) as f64).floor() as i32;
        if let Some(current) = items.get(middle as usize) {
            if *current == k {
                return Some(middle);
            }
            if *current > k {
                high = middle - 1
            }
            if *current < k {
                low = middle + 1
            }
        }
    }
    None
}

struct Find {
    value: i32,
    is_it: bool
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut find: Vec<Find> = vec![];
    let data: Vec<Vec<i32>> = input.split_terminator('\n')
        .map(|x| {
            if x.len() > 11 {
                let a: Vec<i32> = x.split_terminator(" ").map(|x| {
                    let b: i32 = x.parse().unwrap();
                    b
                }).collect();
                a
            } else {
                let a = x.split_terminator(" ")
            }
        }).filter(|x| {
            x.len() != 1
        }).collect();

    for (k, items) in find.iter().zip(data.iter()) {
        let f = binary_search(*k, items);
        dbg!(f);
    }
}
