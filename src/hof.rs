pub fn play() {
    // See https://dev.to/francescoxx/iterators-in-rust-2o0b
    // The Iterator Trait requires that iterators provide a next fn.
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // Closure with a map over an iterator.
    // iter borrows each element and returns ownership
    let a_closure = |&n| n * 100;
    let nums = [1, 2, 3, 4, 5];
    let result: Vec<_> = [1, 2, 3, 4, 5].iter().map(a_closure).collect();
    println!("A closure with map: {:?}", result);
    println!("Original iter: {:?}", nums);

    let upper = 1000;
    // Functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .sum(); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
