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

fn main(){
    let s1 = S{ a:1, b:true };
    let s2: S = S{ a:2, b:true };
    if s1==s2 {
        println!("Equals")
    }else{
        println!("Distinct")
    }
}