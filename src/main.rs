fn main() {
    let mut i = 1;
    while i <= 3 {
        println!("{:?}", i);
        i = i + 1;
    }

    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let (_name, _age) = ("Emma", 20);
}
