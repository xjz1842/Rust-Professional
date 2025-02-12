pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let mut result = "".to_string();
    // 定义一个栈
    let idx = num_str.find('(').unwrap();
   
    let source_num = &num_str[0..idx];
    let source_radix: &str  = &num_str[idx+1..num_str.len()-1];
    // 先转成10进制
    let source_radix = source_radix.parse::<u32>().unwrap();
    let mut ten_radix_num = 0;
    
    for (idx,c) in source_num.chars().rev().enumerate(){
          ten_radix_num  +=  c.to_digit(10).unwrap() * source_radix.pow(idx as u32);
    }
    // 在转成目标进制
    let mut stack = vec![];
    let mut pow = 0;
    // 首先找到最高位置
    while ten_radix_num >=  to_base.pow(pow)  {
        stack.push(to_base.pow(pow));
        pow += 1;
    }
     while !stack.is_empty() {
        let last  =  stack.pop().unwrap();
        let v = ten_radix_num / last;
        ten_radix_num -= v * last;
        if v >= 10 {
            result.push( (b'a' + (v - 10) as u8) as char);
        } else {
            result.push_str(v.to_string().as_str())
        }
     }
    result
}
