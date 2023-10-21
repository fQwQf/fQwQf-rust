fn main() {
    let mut s = String::from("hello");
 
     let r1 = &s;
     let r2 = &s;
     println!("{} and {}", r1, r2);
     // 新编译器中，r1,r2作用域在这里结束
 
     let r3 = &mut s;
     println!("{}", r3);
 } // 老编译器中，r1、r2、r3作用域在这里结束
   // 新编译器中，r3作用域在这里结束