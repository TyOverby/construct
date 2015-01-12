#![allow(unstable)]
use std::collections::dlist::DList;
use std::collections::binary_heap::BinaryHeap;
use std::collections::vec_map::VecMap;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::collections::bitv::Bitv;
use std::collections::bitv_set::BitvSet;
use std::collections::btree_map::BTreeMap;
use std::collections::btree_set::BTreeSet;
use std::collections::ring_buf::RingBuf;

use std::default::Default;
use std::hash::{Hash, Hasher};
use std::collections::hash_state::HashState;

#[macro_export]
macro_rules! construct {
    // HACK HACK HACK
    (COUNT,) => { 0 };
    (COUNT, $first: expr) => { 1 };
    (COUNT, $first: expr, $($rest: expr),+) => {
        1 + construct!(COUNT, $($rest),+)
    };

    ($t: ty, $($elems: expr ),*) => {
        {
            use $crate::Construct;
            let size = construct!(COUNT, $($elems),*);
            let mut coll: $t = Construct::with_cap(size);
            $(coll.add($elems);)*
            coll
        }
    };
}

pub trait Construct {
    type From;

    fn with_cap(capacity: usize) -> Self;
    fn add(&mut self, f: Self::From);
}

impl <T> Construct for Vec<T> {
    type From = T;

    fn with_cap(capacity: usize) -> Vec<T> {
        Vec::with_capacity(capacity)
    }

    fn add(&mut self, f: T) {
        self.push(f);
    }
}

impl <T> Construct for DList<T> {
    type From = T;

    fn with_cap(_: usize) -> DList<T> {
        DList::new()
    }

    fn add(&mut self, f: T) {
        self.push_back(f);
    }
}

impl <T: Ord> Construct for BinaryHeap<T> {
    type From = T;

    fn with_cap(cap: usize) -> BinaryHeap<T> {
        BinaryHeap::with_capacity(cap)
    }

    fn add(&mut self, f: T) {
        self.push(f);
    }
}

impl <T> Construct for VecMap<T> {
    type From = (usize, T);

    fn with_cap(cap: usize) -> VecMap<T> {
        VecMap::with_capacity(cap)
    }

    fn add(&mut self, (k, v) : (usize, T)) {
        self.insert(k, v);
    }
}

impl <K, V, S, H> Construct for HashMap<K, V, S>
where K: Eq + Hash<H>,
      S: HashState<Hasher=H> + Default,
      H: Hasher<Output=u64> {
    type From = (K, V);

    fn with_cap(cap: usize) -> HashMap<K, V, S> {
        HashMap::with_capacity_and_hash_state(cap, Default::default())
    }

    fn add(&mut self, (k, v) : (K, V)) {
        self.insert(k, v);
    }
}

impl Construct for Bitv {
    type From = usize;

    fn with_cap(cap: usize) -> Bitv {
        Bitv::with_capacity(cap)
    }

    fn add(&mut self, e: usize) {
        self.set(e, true);
    }
}

impl Construct for BitvSet {
    type From = usize;

    fn with_cap(cap: usize) -> BitvSet {
        BitvSet::with_capacity(cap)
    }

    fn add(&mut self, e: usize) {
        self.insert(e);
    }
}

impl <K: Ord, V> Construct for BTreeMap<K, V> {
    type From = (K, V);

    fn with_cap(_: usize) -> BTreeMap<K, V> {
        BTreeMap::new()
    }

    fn add(&mut self, (k, v): (K, V)) {
        self.insert(k, v);
    }
}

impl <T: Ord> Construct for BTreeSet<T> {
    type From = T;

    fn with_cap(_: usize) -> BTreeSet<T> {
        BTreeSet::new()
    }

    fn add(&mut self, e: T) {
        self.insert(e);
    }
}


impl <K, S, H> Construct for HashSet<K, S>
where K: Eq + Hash<H>,
      S: HashState<Hasher=H> + Default,
      H: Hasher<Output=u64> {
    type From = K;

    fn with_cap(cap: usize) -> HashSet<K, S> {
        HashSet::with_capacity_and_hash_state(cap, Default::default())
    }

    fn add(&mut self, k: K) {
        self.insert(k);
    }
}

impl <T> Construct for RingBuf<T> {
    type From = T;

    fn with_cap(cap: usize) -> RingBuf<T> {
        RingBuf::with_capacity(cap)
    }

    fn add(&mut self, t: T) {
        self.push_back(t);
    }
}

#[test] fn test_count() {
    assert_eq!(construct!(COUNT, 1,2,3,4), 4);
    assert_eq!(construct!(COUNT,), 0);
    assert_eq!(construct!(COUNT, 2), 1);
}

#[test] fn test_vec() {
    let v = construct!(Vec<_>, 1,2,3,4);
    assert_eq!(v, vec![1,2,3,4]);
}

#[test] fn test_map() {
    let m = construct!(HashMap<_,_>, (1, "hi"), (2, "bye"));

    let mut manual = HashMap::new();
    manual.insert(1, "hi");
    manual.insert(2, "bye");
    assert_eq!(m, manual);
}

