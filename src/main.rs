fn main() {
   // 直接隐式定义一个 tuple 类型
   let test_tuple1 = ("qqqq",1,true);
   let mut  test1 = ("qqqq",1,true);
   
   test1.1 = 42;

   //  显示定义和显示 tuple 类型
   let test_tuple2 :(&str,i32,bool,[i32;5]) = ("aaaa",1234,false,[1,2,3,4,5]);
   //  为了输出显示，一样需要使用 debug 进行打印结果
   println!("test tuple : {test_tuple1:?}");
   println!("test tuple 2: {test_tuple2:?}");
   println!("test1 {test1:?}");


   // special type of the array
   let numbers_slice:&[i32] = &[1,2,3,4,5];
   println!("the numbers of the slice {numbers_slice:?}");
   
}


