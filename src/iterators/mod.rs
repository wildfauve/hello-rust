pub mod value;
use value::Shoe;

pub fn play() {
    // Iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for i in v1_iter {
        println!("Iterator val: {i}");
    }

    // Iterators implement the Iterator trait.  Basically, the Iterator defines a type
    // which is the type to be returned from the next() fn.  The next() method must return a Option<Type-of-Item>
    let v2 = vec![1, 2, 3];

    let mut v2_iter = v2.iter(); // next() will consume the iterator, hence v1_iter needs to be mut.

    assert_eq!(v2_iter.next(), Some(&1)); // vals from next() are immut references to the vals in the vector.
    // if we want to take ownershop of v2, call into_iter()
    // if we want mut ref, call iter_mut()
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);
    assert_eq!(v2_iter.next(), None);

    // when the iter is used up the consumer is called a comsumer-adapter
    let v3_iter = v1.iter();
    let total: i32 = v3_iter.sum(); // sum takes ownership of v3_iter, thus it is no longer available.
    println!("Total: {total}");
    // println!("Total: {v3_iter:?}"); // this wont compile.

    // Iterator-Adapters dont consume the iter; they return a new iterator.
    let v2 = vec![1, 2, 3];
    v2.iter().map(|x| x + 1); // this is also lazy, so nothing happens; collect() is a consuming-adapter however.
    let v2_plus_1: Vec<_> = v2.iter().map(|x| x + 1).map(|x| x * 10).collect();
    println!("Iter with map: {v2_plus_1:?}");

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("Shoes in my size: {in_my_size:?}");
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // Takes ownership of the shoes vector.
    // filter is, well, filter!
    // into_iter() takes ownership
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
