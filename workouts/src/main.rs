// TODO impl with hashmap to allow multiple values

// TODO impl with generics to allow other types of function parameters

struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                println!("cache hit {}", arg);
                v
            },
            None => {
                println!("calculating {}...", arg);
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn main() {
    let mut c10 = Cacher::new(|n| n+1);
    let mut c8 = Cacher::new(|n| n+1);
    println!("{}", c10.value(10));
    println!("{}", c10.value(10));
    println!("{}", c8.value(8));
    println!("{}", c8.value(8));
    println!("{}", c8.value(27));
}


#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

