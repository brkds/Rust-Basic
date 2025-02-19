fn main() {
    let mut s = String::from("111");
    let  mut s1 = String::from("222");
    let s3 = & mut s1;
    let s4 = &s1;
    println!("{} {}",s4);
}
