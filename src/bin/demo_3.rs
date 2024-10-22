trait Fall {
    fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("the vase broken!");
    }
}

struct Cat;
impl Fall for Cat {
    fn hit_ground(&self) {
        println!("the cat casually walked away");
    }
}

fn fall(thing: impl Fall) {
    thing.hit_ground();
}

fn main() {
    fall(Vase {});
    fall(Cat {});
}