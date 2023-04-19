trait Print {
    fn print(&self);
}
struct S { i: i32 }
impl Print for S {
    fn print(&self){
        println!("S {}", self.i);
    }
}

fn process(v: &dyn Print){ //reference a qualcosa che implementa Print; & dava errore, dyn serve a noi a sapere che e' un fat reference
//fat pointer contiene 8byte che puntano alla struttura dati. I secondi 8byte puntano alla v table del tratto.
    v.print();
}

fn main() {
    process(&S{i:0})
}
