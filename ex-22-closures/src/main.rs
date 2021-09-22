fn main() {
    let add = |a, b| a + b;
    let sub = |a: i32, b: i32| a - b;

    let x = 3;
    // x is not accessible inside function
    // fn pow(n: i32) -> i32 {
    //     let mut res: i32 = 1;
    //     for _ in 0..x {
    //         res = res * n;
    //     }
    //     res
    // }
    // help: use the `|| { ... }` closure form instead

    // type: fn(i32) -> i32
    let cube = |n: i32| {
        let mut res: i32 = 1;
        // x is from outside
        for _ in 0..x {
            res *= n;
        }
        res
    };

    println!("3 + 8 = {}", add(3, 8));
    println!("56 - 78 = {}", sub(56, 78));
    println!("cube of 89 = {}", cube(89));

    let calc = |n: u32| n * 3 - 10;
    let mut rc = RandomCalculation::new(calc);
    println!("RandomCalculation 89 = {}", rc.value(89));
}

struct RandomCalculation<T>
where
    T: Fn(u32) -> u32,
    // other closure types:
    // FnMut(..)
    // FnOnce(..)
{
    calculation: T,
    value: Option<u32>,
}

impl<T> RandomCalculation<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> RandomCalculation<T> {
        RandomCalculation {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, value: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(value);
                self.value = Some(v);
                v
            }
        }
    }
}
