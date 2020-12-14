use std::{
    io::{self, Read},
    unreachable,
};

// Way too slow for actual input.
fn naive(buses: &Vec<(i64, i64)>, first_id: i64) -> i64 {
    for i in 0..i64::MAX {
        let mut ok = true;
        let t = i * first_id;
        for (n, id) in buses {
            if (t + n) % id != 0 {
                ok = false;
                break;
            }
        }

        if ok {
            return t;
        }
    }

    unreachable!()
}

// assume a and b are prime
fn recurrence_period(a: i64, b: i64, n: i64) -> i64 {
    a * b + (n - 1)
}

fn main() {
    let mut buses: Vec<(i64, i64)> = Vec::new();
    let mut first_id = 0;

    let input: Vec<String> = get_input().lines().map(|s| s.to_string()).collect();
    for (n, s) in input[1].split(",").enumerate() {
        if s == "x" {
            continue;
        }

        let id: i64 = s.parse().unwrap();
        if first_id == 0 {
            first_id = id;
        }
        buses.push((n as i64, id));
    }

    let res = buses
        .iter()
        .fold(1, |accum, &(n, id)| recurrence_period(accum, id, n));

    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
