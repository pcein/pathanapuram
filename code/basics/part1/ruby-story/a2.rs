
fn main() {
    let v = vec![1,2,3,4];
    
    let p = v.iter().any(|&x| x > 5);

    println!("{}", p);
}
