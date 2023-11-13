#![cfg(test)]

use arrowpipe::Arrow;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn double(x: i32) -> i32 {
    x * 2
}

fn subtract_one(x: i32) -> i32 {
    x - 1
}

#[test]
fn test_new() {
    let arrow = Arrow::new(add_one);
    assert_eq!(arrow.apply(1), 2);
}

#[test]
fn test_symbiotize() {
    let mut arrow = Arrow::new(add_one);
    arrow.symbiotize(Arrow::new(double));
    assert_eq!(arrow.shoot(1), 4);
}

#[test]
fn test_shoot() {
    let mut arrow = Arrow::new(add_one);
    arrow.symbiotize(Arrow::new(double));
    arrow.symbiotize(Arrow::new(subtract_one));
    assert_eq!(arrow.shoot(1), 3);
}

#[test]
fn test_shoot_reverse() {
    let mut arrow = Arrow::new(add_one); // First 1 -> 2
    arrow.symbiotize(Arrow::new(double)); // Third 1 -> 2
    arrow.symbiotize(Arrow::new(subtract_one)); // Second 2 -> 1
    assert_eq!(arrow.shoot_reverse(1), 2);
}

#[test]
fn test_clear() {
    let mut arrow = Arrow::new(add_one);
    arrow.symbiotize(Arrow::new(double));
    arrow.clear();
    assert_eq!(arrow.shoot(1), 2);
}

#[test]
fn test_remove() {
    let mut arrow = Arrow::new(add_one); // First, 1 -> 2
    let idx = arrow.symbiotize(Arrow::new(double));
    arrow.symbiotize(Arrow::new(subtract_one)); // Second, 2 -> 1
    arrow.remove(idx);
    assert_eq!(arrow.shoot(1), 1);
}

#[test]
fn test_nop() {
    let arrow = Arrow::nop();
    arrow.shoot(())
}

#[test]
fn test_from() {
    let arrow = Arrow::<_, usize>::default();
    arrow.shoot(1u8);
}

#[test]
fn weird_structs() {
    struct NotCopy {
        x: Vec<i8>,
        s: String,
    }

    let mut arrow = Arrow::new(|i: NotCopy| i.x.len() + i.s.len());
    arrow.symbiotize(Arrow::new(|i: usize| i * 2));

    let nc = NotCopy {
        x: vec![1, 2, 3],
        s: "123".to_string(),
    };

    assert_eq!(arrow.shoot(nc), 12);
}
