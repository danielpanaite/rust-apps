mod main2;
use main2::main2;

struct S{
    a: i32,
    b: bool
}

impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        return self.a == other.a &&
        self.b == other.b
    }
}

fn main1(){
    let s1 = S{ a:1, b:true };
    let s2: S = S{ a:2, b:true };
    if s1==s2 {
        println!("Equals")
    }else{
        println!("Distinct")
    }
}

fn main(){
    println!("Main 1");
    main1();

    println!("Main 2");
    main2();
}