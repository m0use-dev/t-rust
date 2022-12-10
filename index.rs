fn main() {
  // 出力
  println!("Hello, world!");

  // 変数
  let	num= 11;

  // 配列
  let name = ["山田", "佐々木", "田中"];
  println!("{} {} {}", name[0], name[1], name[2]);

  // 条件分岐
  if num > 10 {
    println!("大きい");
  } else if num == 10 {
      println!("同じ");
  } else {
      println!("小さい");
  }

  // 繰り返し
  for i in 0..10 {
    println!("{}", i);
  }
}
