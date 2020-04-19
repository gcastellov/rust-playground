pub fn some_borrowings() {
    let a: u8 = 10;
    let b: u8 = a;
    println!("This is allowed so b is: {}", b);

    let c = vec![1,2,3];
    let d = &c;
    println!("This is allowed because d owns the reference now: {}. However c is not accessible", d[0]);

    let e = vec![5,6,7];
    {
        let f = &e;
        println!("This is allowed because f owns the reference now: {}", f[0]);
    }
    println!("This is allowed because scope has finished and e is again available: {}", e[0]);
}