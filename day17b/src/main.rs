use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

type Pos = (i64, i64, i64, i64);

fn main() {
    let mut active: HashSet<Pos> = HashSet::new();
    for (y, line) in get_input().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert((x as i64, y as i64, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        let mut next_active: HashSet<Pos> = HashSet::new();
        let mut neighbors_for_inactives: HashMap<Pos, usize> = HashMap::new();

        for (x, y, z, w) in active.iter() {
            let current = (*x, *y, *z, *w);
            let mut active_neighbors = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in -1..=1 {
                            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                continue;
                            }

                            let other = (x + dx, y + dy, z + dz, w + dw);

                            if active.contains(&other) {
                                active_neighbors += 1;
                            } else {
                                *neighbors_for_inactives.entry(other).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }

            // Survival check
            if active_neighbors == 2 || active_neighbors == 3 {
                next_active.insert(current);
            }
        }

        // Spawn check
        for (pos, active_neighbors) in neighbors_for_inactives.iter() {
            if *active_neighbors == 3 {
                next_active.insert(*pos);
            }
        }

        active = next_active;
    }

    println!("{}", active.len());
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
