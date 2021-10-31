fn main() {
    let mut vector: Vec<u32> = Vec::new();
    //集合的值
    vector.push(100);
    vector.push(32);
    vector.push(64);
    println!("列表数据{:?}",vector);

    //计算得到结合整数之和
    let result = sum(vector);
    println!("sum : {}",result);
}

fn sum(v: Vec<u32>) -> u32{
  let mut result = 0;
  for i in v {
    result = result + i;
  }

  let opt : Option<u32> = Some(result);

  match opt {
     Some(value) => {
        return value;
     },
     None => {
        return 0;
      }
  }
}
