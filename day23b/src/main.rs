fn main() {
    let maxval = 1_000_000;
    let num_rounds = 10_000_000;
    let mut nexts = vec![0; maxval + 1];

    // insert initial values
    let input = "562893147";
    let mut initial_vals = input.chars().map(|c| c.to_digit(10).unwrap() as usize);
    let mut cur = initial_vals.next().unwrap();
    let first = cur;

    for next in initial_vals {
        nexts[cur] = next;
        cur = next;
    }

    // backfill the remaining values
    for next in (input.len() + 1)..=maxval {
        nexts[cur] = next;
        cur = next;
    }

    // complete the ring
    nexts[cur] = first;

    cur = first;
    for _ in 0..num_rounds {
        let move1 = nexts[cur];
        let move2 = nexts[move1];
        let move3 = nexts[move2];

        // calculate destination
        let mut dest = cur;
        while [cur, move1, move2, move3].contains(&dest) {
            dest -= 1;
            if dest == 0 {
                dest = maxval;
            }
        }

        // move some stuff around
        nexts[cur] = nexts[move3];
        let temp = nexts[dest];
        nexts[dest] = move1;
        nexts[move3] = temp;

        // advance current pointer
        cur = nexts[cur];
    }

    println!("{}", nexts[1] * nexts[nexts[1]]);
}
