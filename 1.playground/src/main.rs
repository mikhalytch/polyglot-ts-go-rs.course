/* fn main() {
    /* let mut a = vec![];
    {
        let mut another = a;

        another.push(1);
        println!("{:?}", another);
    }
    a.push(1);

    println!("{:?}", a); */
    // println!("{:?}", another);
    //

    /* let x = 5;
    let y = x;
    println!("{:?}", x + y); */

    /* let x = 5;
    let mut y = &x;
    let z = &mut x;

    y = &6;

    println!("x is {:?} and y is {:?}", x, y);
    println!("{:?}", x + y); */

    /* let mut x = 5;
    let z = &mut x;

    *z = 7;

    // println!("x is {:?} and z is {:?}", x, z);
    println!("{:?}", z);

    x = 9;
    println!("{:?}", x); */

    /* let mut a: Vec<i32> = vec![];
    let b = &a;
    let c = &mut a;

    println!("{:?} {:?} {:?}", a, b, c); */

    let foo = RSEnum::Foo(5);

    if let RSEnum::Foo(value) = foo {
        println!("{}", value);
    }

    match foo {
        RSEnum::FooOpt(Some(value)) => {
            println!("{}", value);
        }
        RSEnum::FooOpt(None) => {}
        RSEnum::Foo(value) => {
            println!("{}", value);
        }
        RSEnum::FooFn(value) => {
            println!("{:?}", value);
        }
        RSEnum::Bar(_) => todo!(),
        RSEnum::Baz(_) => todo!(),
        RSEnum::FooOpt(_) => todo!(),
        RSEnum::FooRec(_) => todo!(),
    }
}

enum RSEnum {
    FooOpt(Option<i32>),
    Foo(i32),
    FooFn(fn() -> RSEnum),
    // FooRec(RSEnum),
    Bar(String),
    Baz(Vec<String>),
} */

fn main() {
    let foo = Some(5);

    /* if let Some(value) = foo {
        println!("{}", value);
    } */
    if let Some(value) = foo {
        println!("{}", value);
    }

    match foo {
        Some(5) => println!("was 5"),
        Some(value) => {
            println!("was unexpected {}", value);
        }
        None => todo!(),
    }

    // TS:
    // foo.map(() => {
    //   ...
    // });
    foo.filter(|x| x < &2)
        .map(|x| println!("mapped value: {}", x));
    // map(|x| {
    // println!("mapped value: {}", x);
    // });

    for c in "ldskfjsdfsdf".chars() {
        println!("{}", c);
    }

    let a: Vec<i32> = vec![];
    let _filter = a.iter().filter(|x| {
        println!("{}", x);
        true
    });

    let a = 5;
    let b = &a;
    let c = &5;

    assert_eq!(b, c);
    // assert_eq!(a, c);
}
