use std::{
    collections::HashMap,
    io::{self, Read},
};

use regex::Regex;
#[derive(Debug, Copy, Clone, PartialEq)]
enum Bit {
    Zero,
    One,
    Float,
}

#[derive(Debug, Clone)]
enum Op {
    SetMask(Vec<Bit>),
    AssignMem(usize, u64),
}

fn set_bit(v: usize, bit: usize) -> usize {
    v | 1 << bit
}

fn clear_bit(v: usize, bit: usize) -> usize {
    v & !(1 << bit)
}

fn distribute_bits(mut addr: usize, bitvec: &Vec<bool>, targets: &Vec<usize>) -> usize {
    for (i, b) in bitvec.iter().enumerate() {
        let target_bit = targets[i];
        addr = if *b {
            set_bit(addr, target_bit)
        } else {
            clear_bit(addr, target_bit)
        };
    }
    addr
}

fn to_bitvec(bits: u64, num_bits: usize) -> Vec<bool> {
    (0..num_bits).fold(Vec::new(), |mut accum, b| {
        accum.push((bits & (1 << b)) == 0);
        accum
    })
}

fn munge_addr(addr: usize, mask: &Vec<Bit>) -> Vec<usize> {
    // Process all Bit::One, which forces bits to be set
    let mut base_addr = addr;
    for (i, b) in mask.iter().enumerate() {
        if *b == Bit::One {
            base_addr = set_bit(base_addr, i);
        }
    }

    // Generate all possible floating bits
    let floating_bits: Vec<usize> = mask
        .iter()
        .enumerate()
        .filter(|(_i, b)| **b == Bit::Float)
        .map(|(i, _b)| i)
        .collect();

    let mask_size = floating_bits.len();
    let mut addrs: Vec<usize> = Vec::new();
    for mask in 0..(1 << mask_size) {
        let mask_vec = to_bitvec(mask, mask_size);
        addrs.push(distribute_bits(base_addr, &mask_vec, &floating_bits));
    }

    addrs
}

fn main() {
    let mask_re = Regex::new(r#"mask = ([X01]{36})"#).unwrap();
    let assign_re = Regex::new(r#"mem\[(\d+)\] = (\d+)"#).unwrap();

    let mut program: Vec<Op> = Vec::new();
    for line in get_input().lines() {
        if let Some(caps) = mask_re.captures(line) {
            let mut mask = Vec::new();
            for c in caps[1].chars().rev() {
                mask.push(match c {
                    '0' => Bit::Zero,
                    '1' => Bit::One,
                    'X' => Bit::Float,
                    _ => unreachable!(),
                });
            }
            program.push(Op::SetMask(mask.clone()));
        } else if let Some(caps) = assign_re.captures(line) {
            let addr: usize = caps[1].parse().unwrap();
            let val: u64 = caps[2].parse().unwrap();
            program.push(Op::AssignMem(addr, val));
        } else {
            panic!("could not parse {}", line);
        }
    }

    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut current_mask = Vec::new();
    for op in program {
        match op {
            Op::SetMask(m) => current_mask = m,
            Op::AssignMem(addr, val) => {
                for a in munge_addr(addr, &current_mask) {
                    mem.insert(a, val);
                }
            }
        }
    }

    let res = mem.iter().fold(0, |accum, (_, v)| accum + v);
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
