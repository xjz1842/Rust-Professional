pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let mut p = 1.0000; 
    for i in 1..n {
        p *= (365 - i) as f64 / 365.0;
    }
    let result = 1.0 - p ; 
    let result =  format!("{:.4}",result);
    result.parse::<f64>().unwrap()
}
