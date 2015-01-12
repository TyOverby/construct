#![allow(unstable)]

pub struct ReserveN<T> {
    pub n: usize
}

impl <T> Iterator for ReserveN<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.n, Some(self.n))
    }
}

pub struct Single<T> {
    t: Option<T>
}

impl <T> Single<T> {
    pub fn new(t: T) -> Single<T> {
        Single {t: Some(t)}
    }
}

impl <T> Iterator for Single<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.t.take()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let s = match self.t {
            Some(_) => 1,
            None => 0
        };
        (s, Some(s))
    }
}

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
            use std::iter::Extend;
            use std::default::Default;

            let size = construct!(COUNT, $($elems),*);

            let mut coll: $t = Default::default();
            coll.extend($crate::ReserveN{n: size});
            $(coll.extend($crate::Single::new($elems));)*
            coll
        }
    };
}

#[cfg(test)]
mod test {
    use std::collections::hash_map::HashMap;

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
}
