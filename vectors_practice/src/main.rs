fn main() {
/*    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v); // Displaying properly a vector*/

    /*let v =vec![1,2,3,4,5];
    // let does_not_exist = &v[100];
    // the code below will not panic
    let does_not_exist2 = v.get(100);*/

    let mut v = vec![1,2,3,4,5];

    //let first = &v[0]; //don't put here it will become invalid
    v.push(6);
    let first = &v[0];
    println!("The first element is: {first}.")

}
