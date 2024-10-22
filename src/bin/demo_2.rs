mod greet {
    fn hello() {
        println!("hello");
    }

    fn goodbye() {
        println!("goodbye");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

trait Fall {
    fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("the vase broken");
    }
}

struct Cat;
impl Fall for crate::Cat {
    fn hit_ground(&self) {
        println!("the cat casually walked away");
    }
}

fn main() {
    use greet::*;
    // math::add(1, 2);

}

#[cfg(test)]
mod test {
    #[test]
    fn check_result() {
        use crate::math;
        let ret = math::add(1, 2);
        assert_eq!(ret, 5, "invalid result");
    }
}