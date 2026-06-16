use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    println!("1. BTREEMAP — sorted key-value");

    let mut btree = BTreeMap::new();
    btree.insert("zebra", 3);
    btree.insert("apel", 1);
    btree.insert("mangga", 2);
    // BTreeMap otomatis sorted by key
    for (k, v) in &btree {
        println!("  {} -> {}", k, v);
    }

    // range query — BTreeMap bisa cari range
    println!("  range [a..n):");
    for (k, v) in btree.range("a".."n") {
        println!("    {} -> {}", k, v);
    }

    println!("\n2. BTREEMAP vs HASHMAP");
    let mut hash = HashMap::new();
    hash.insert("zebra", 3);
    hash.insert("apel", 1);
    hash.insert("mangga", 2);
    // HashMap urutannya random
    println!("  HashMap (random order):");
    for (k, v) in &hash {
        println!("    {} -> {}", k, v);
    }

    // HashMap lebih cepet O(1), BTreeMap O(log n) tapi sorted
    // HashMap butuh Hash trait, BTreeMap butuh Ord trait

    println!("\n3. BTREESET & HASHSET");
    let mut bset = BTreeSet::new();
    bset.insert("apel");
    bset.insert("mangga");
    bset.insert("jeruk");
    bset.insert("apel"); // duplikat, diabaikan
    println!("  BTreeSet: {:?}", bset);

    // HashSet — sama kek BTreeSet tapi lebih cepet, urutan random
    let mut hset = HashSet::new();
    hset.insert("apel");
    hset.insert("mangga");
    hset.insert("jeruk");
    println!("  HashSet: {:?}", hset);

    println!("\n4. SET OPERATIONS");
    let set_a: HashSet<i32> = [1, 2, 3, 4].iter().copied().collect();
    let set_b: HashSet<i32> = [3, 4, 5, 6].iter().copied().collect();

    println!("  A: {:?}", set_a);
    println!("  B: {:?}", set_b);

    let union: HashSet<_> = set_a.union(&set_b).copied().collect();
    println!("  union: {:?}", union);

    let intersection: HashSet<_> = set_a.intersection(&set_b).copied().collect();
    println!("  intersection: {:?}", intersection);

    let difference: HashSet<_> = set_a.difference(&set_b).copied().collect();
    println!("  A - B: {:?}", difference);

    let sym_diff: HashSet<_> = set_a.symmetric_difference(&set_b).copied().collect();
    println!("  symmetric diff: {:?}", sym_diff);

    // subset / superset
    let set_c: HashSet<i32> = [1, 2].iter().copied().collect();
    println!("  C subset A: {}", set_c.is_subset(&set_a));
    println!("  A superset C: {}", set_a.is_superset(&set_c));
    println!("  disjoint A & B: {}", set_a.is_disjoint(&set_b));

    println!("\n5. VECDEQUE — double-ended queue");
    let mut deque: VecDeque<i32> = VecDeque::new();

    // push dari depan & belakang
    deque.push_back(3);
    deque.push_back(4);
    deque.push_front(2);
    deque.push_front(1);
    println!("  after pushes: {:?}", deque);

    // pop dari depan & belakang
    let depan = deque.pop_front();
    let belakang = deque.pop_back();
    println!("  pop front: {:?}, pop back: {:?}", depan, belakang);
    println!("  remaining: {:?}", deque);

    // VecDeque juga bisa indexing & slicing
    if let Some(&val) = deque.get(0) {
        println!("  get(0): {}", val);
    }

    // rotate — muter elemen
    let mut deque2: VecDeque<i32> = (1..=5).collect();
    println!("  before rotate: {:?}", deque2);
    deque2.rotate_left(2);
    println!("  rotate_left(2): {:?}", deque2);
    deque2.rotate_right(1);
    println!("  rotate_right(1): {:?}", deque2);

    // VecDeque vs Vec:
    // Vec: push/pop dari belakang cepet, dari depan lambat (O(n))
    // VecDeque: push/pop dari depan & belakang cepet (O(1))
    // Tapi VecDeque ga bisa di-slice ke &[T]

    println!("\n6. BINARYHEAP — priority queue (max-heap)");
    let mut heap = BinaryHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(10);
    heap.push(3);

    // peek — liat nilai terbesar tanpa pop
    println!("  peek max: {:?}", heap.peek());

    // pop selalu ngambil nilai TERBESAR
    while let Some(val) = heap.pop() {
        println!("  pop: {}", val);
    }

    // Min-heap pake std::cmp::Reverse
    use std::cmp::Reverse;
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(10));
    println!("  Min-heap:");
    while let Some(Reverse(val)) = min_heap.pop() {
        println!("    pop: {}", val);
    }

    // BinaryHeap dari Vec
    let data = vec![3, 1, 4, 1, 5, 9];
    let heap_from_vec: BinaryHeap<i32> = data.into_iter().collect();
    println!("  from vec, peek: {:?}", heap_from_vec.peek());

    println!("\n7. LINKEDLIST — doubly linked list");
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(2);
    list.push_back(3);
    list.push_front(1);
    println!("  after push: {:?}", list);

    // split / append — operasi linked list yang efisien
    let mut list2: LinkedList<i32> = LinkedList::new();
    list2.push_back(4);
    list2.push_back(5);

    list.append(&mut list2);
    println!("  after append list2: {:?}", list);
    println!("  list2 (sekarang kosong): {:?}", list2);

    // split_off
    let mut list3: LinkedList<i32> = (1..=6).collect();
    let tail = list3.split_off(3);
    println!("  list3: {:?}", list3);
    println!("  tail: {:?}", tail);

    // LinkedList jarang dipake — Vec/ VecDeque hampir selalu lebih cepet
    // karena cache locality. LinkedList cepet kalo sering split/merge di tengah.

    println!("\n8. PERFORMANCE COMPARISON");
    println!("  HashMap<K,V>  : O(1) lookup, unordered, Hash trait");
    println!("  BTreeMap<K,V> : O(log n) lookup, sorted, Ord trait");
    println!("  HashSet<T>    : O(1) set operations, Hash trait");
    println!("  BTreeSet<T>   : O(log n) set operations, sorted, Ord trait");
    println!("  Vec<T>        : push/pop back O(1), insert/remove front O(n)");
    println!("  VecDeque<T>   : push/pop front & back O(1)");
    println!("  BinaryHeap<T> : max-heap, push O(1), pop O(log n)");
    println!("  LinkedList<T> : split/merge di tengah O(1), butuh alloc per node");
}
