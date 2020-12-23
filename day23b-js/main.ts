interface Node {
  label: number,
  next: Node | undefined,
}

interface Ring {
  cur: Node,
  byLabel: Node[]
}

function makeRing(initial: number[], max: number): Ring {
  const byLabel = new Array(max)
  
  let node = { label: initial[0], next: undefined }
  byLabel[initial[0] - 1] = node

  for (const label of initial.slice(1)) {
    node.next = { label, next: undefined }
    byLabel[label - 1] = node.next
    node = node.next
  }

  for (let label = initial.length + 1; label <= max; label++) {
    node.next = { label, next: undefined }
    byLabel[label - 1] = node.next
    node = node.next
  }

  node.next = byLabel[initial[0] - 1]

  return {
    cur: byLabel[initial[0] - 1],
    byLabel,
  }
}

function round(ring: Ring) {
  let move1 = ring.cur.next
  let move2 = move1.next
  let move3 = move2.next

  const denylist = [
    ring.cur.label,
    move1.label,
    move2.label,
    move3.label
  ]

  let dest = ring.cur.label;
  while (denylist.indexOf(dest) >= 0) {
    dest--;
    if (dest === 0) {
      dest = ring.byLabel.length
    }
  }

  ring.cur.next = move3.next
  
  const before = ring.byLabel[dest - 1] // zero-indexed
  const after = before.next
  before.next = move1
  move3.next = after
  
  ring.cur = ring.cur.next
}

const ring = makeRing([5, 6, 2, 8, 9, 3, 1, 4, 7], 1e6)

for (let i = 0; i < 1e7; i++) {
  round(ring)
}

let node1 = ring.byLabel[0]
console.log(node1.next.label * node1.next.next.label)

