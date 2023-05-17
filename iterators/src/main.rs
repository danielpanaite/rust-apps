#[derive(Debug)]
struct T(i32);
impl Drop for T{
    fn drop(&mut self) { //function to check when it disappears
        println!("Dropping T({})", self.0);
    }
}

fn main() {

    let v = vec![T(1), T(2), T(3)];

    for i in v.iter() { //without .iter the cycle takes ownership
        println!("{}", i.0);
    }

    println!("Final value of v: {:?}", v);

    // for i in v.into_iter() {
    //     println!("{}", i.0);
    // } //into_iter breaks the vector and drops after every item

    //println!("Final value of v: {:?}", v);
    //does not work

    println!("Fine");

} //items of .iter cycle are dropped here
