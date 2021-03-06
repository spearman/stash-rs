#![feature(test)]

extern crate stash;
extern crate test;

use test::Bencher;

use stash::Stash;

#[bench]
fn bench(b: &mut Bencher) {
    let mut stash = Stash::new();
    b.iter(|| {
        let t1 = stash.put("something");
        let t2 = stash.put("something");
        let _ = stash.take(t1);
        let t3 = stash.put("something");
        let t4 = stash.put("something");
        let t5 = stash.put("something");
        let _ = stash.take(t4);
        let t6 = stash.put("something");
        let _ = stash.take(t3);
        let _ = stash.take(t2);
        let _ = stash.take(t5);
        let _ = stash.take(t6);
    });
}

#[bench]
fn lookup(b: &mut Bencher) {
    let mut stash = Stash::new();
    let mut tickets = Vec::new();
    for _ in 0..100 {
        tickets.push(stash.put("something"));
    }
    let t = &tickets[20];
    b.iter(|| {
        test::black_box(&stash[*t]);
    });
}

#[bench]
fn iter_sparse(b: &mut Bencher) {
    let mut stash = Stash::new();
    let mut tickets = Vec::new();
    for _ in 0..100 {
        tickets.push(stash.put("something"));
    }
    stash.put("something");
    for t in tickets {
        stash.take(t);
    }
    b.iter(|| {
        test::black_box(test::black_box(&stash).iter().next().unwrap());
    });
}

#[bench]
fn iter(b: &mut Bencher) {
    let mut stash = Stash::new();
    for _ in 0..100 {
        stash.put("something");
    }
    b.iter(|| {
        for i in test::black_box(&stash) {
            test::black_box(i);
        }
    });
}

#[bench]
fn insert_delete(b: &mut Bencher) {
    let mut stash = Stash::new();

    for _ in 0..100 {
        stash.put("something");
    }

    stash.take(10);
    stash.take(50);
    stash.take(20);

    b.iter(|| {
        let index = stash.put(test::black_box("something"));
        test::black_box(stash.take(test::black_box(index)));
    });
}
