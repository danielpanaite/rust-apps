fn create_fun(even: bool, base: i32) -> Box<dyn Fn(i32) -> i32> { //funzione che ritorna una funzione che ritorna un altro intero
    //ritorno Box perche la funzione potrebbe avere dimensione diversa
    if even {
        return Box::new(move |i:i32| { i-(i%2)+base }) //ritorno il valore per reference ma non posso perche non ha una lifetime oltre alla closure. Devo usare move
    }else{
        return Box::new(move |i:i32| { i-(i%2)+1+base })
    }
}  

fn generator(base: &str) -> impl FnMut() -> String+ '_ { //catturo un &str quindi potenzialmente ho un tempo di vita
    let mut v: i32 = 0;
    return move|| { v+=1; format!("{}{}", base, v) }
}


fn main() {
    println!("Closures!");

    let mut g = generator("alfa");
    println!("{}", g());
    println!("{}", g());
    println!("{}", g());
    println!("{}", g());

    let mut h = generator("beta");
    println!("{}", h());
    println!("{}", h());
    println!("{}", h());
    println!("{}", h());
    println!("{}", g());
    //l'oggetto si ricorda quante volte e' stato chiamato

    //let fx = |x|{ x + y.f() + z } //contiene solo puntatori
    //let fy = move |x|{ x + y.f() + z }//contiene i valori 

    let f1 = create_fun(true, 100);
    for i in 0..10 {
        println!("f1({}): {}", i, f1(i));
    }

    let f2 = create_fun(true, 200);
    for i in 0..10 {
        println!("f1({}): {}", i, f2(i));
    }
}
