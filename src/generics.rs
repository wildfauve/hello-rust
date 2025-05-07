trait SomeTrait {
    fn blah(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct SomeStruct {
    something: i32,
}

impl SomeTrait for SomeStruct {
    fn blah(&self, a: &str, b: &str) -> String {
        self.something.to_string() + "-" + a + "-" + b
    }
}

impl SomeTrait for i32 {
    fn blah(&self, a: &str, b: &str) -> String {
        "i32".to_string() + "-" + a + "-" + b
    }
}

// We might want points of i32, f32, etc
// The <T> is just a convertion.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MultiTypePoint<T, U> {
    x: T,
    y: U,
}

// Notice that the constraint is, here, on the impl, not the struct
impl<T, U> MultiTypePoint<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn log_something(&self) {
        println!("Log: {:?}, {:?}", self.x, self.y);
    }
}

// Generics can also have concrete types
struct ConcretePoint<T> {
    x: T,
    y: i32,
}

// Enums with item data as generic
enum GenericEnum<T> {
    Option1(T),
    Option2(T),
    Option3,
}

pub fn play() {
    let point_i32 = Point { x: 10, y: 50 };
    println!("I32 Struct: {:?}", point_i32);

    let point_f32 = Point { x: 10.1, y: 50.1 };
    println!("f32 Struct: {:?}", point_f32);

    let multi_type_point_i32_f32 = MultiTypePoint { x: 10, y: 50.1 };
    println!("i32_f32 Struct: {:?}", multi_type_point_i32_f32);

    let option1 = GenericEnum::Option1("a str");

    // calling generic fns
    let generic_fn_result_1 = generic_fn(1, 2);
    let generic_fn_result_2 = generic_fn(1.1, 2.1);

    let some_val = SomeStruct { something: 1000 };
    let result1 = using_the_custom_trait(&some_val);
    println!("with result {}", result1);

    // But i32 does not does not work with using_custom_trait as its not a SomeStruct
    // But if we implement blah on i32.....when it will compile
    // Therefore types are not important in generic programming, traits are.  A generic fn could work
    // with any type, as long as it satisfies the constraints!
    let result2 = using_the_custom_trait(&10);
    println!("with result {}", result2);

    // Using Generics on the Impl
    let m_point = MultiTypePoint {
        x: 10,
        y: vec![1, 2, 3],
    }; // that vec! is a vector macro
    m_point.log_something();
}

//generic fns
fn generic_fn<T: std::ops::Add<Output = T> + std::fmt::Debug>(arg1: T, arg2: T) -> T {
    // Adding 2 type T's together may not make sense.  We need to add a trait constraint to type T
    // to constrain it to types that can be added (this is the std::ops::Add trait)
    // We need the Output=T because the addition of 2 T's may result in a new type.
    // We can add multiple constaints....like Debug...
    println!("arg1: {:?}, args: {:?}", arg1, arg2);
    arg1 + arg2
}

// There is another way of writing generic_fn so that the formatting is better....the where clause.
fn generic_fn_2<T, X>(arg1: T, arg2: T, arg3: X) -> T
where
    T: std::ops::Add<Output = T> + std::fmt::Debug,
    X: std::fmt::Debug,
{
    // Adding 2 type T's together may not make sense.  We need to add a trait constraint to type T
    // to constrain it to types that can be added (this is the std::ops::Add trait)
    // We need the Output=T because the addition of 2 T's may result in a new type.
    // We can add multiple constaints....like Debug...
    println!("arg1: {:?}, args: {:?} args: {:?}", arg1, arg2, arg3);
    arg1 + arg2
}

fn using_the_custom_trait<T>(some_val: &T) -> String
where
    T: SomeTrait + std::fmt::Debug,
{
    // complicated code here
    println!("In using custom trait: {:?}", some_val);
    some_val.blah("str1", "str2")
}
