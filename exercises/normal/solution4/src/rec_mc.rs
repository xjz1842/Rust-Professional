pub fn dp_rec_mc(amount: u32) -> u32 {
    // 表示0..i区间内，使用最少的钱组成dp[i]
    let mut dp = vec![0; (amount+1) as usize];
    
    let avaible_amount : [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
 
    for money in avaible_amount {
       for i in money..=amount {
           if i == money {
              dp[i as usize] = 1;
           } else {
               let mut count = 1;
               let mut j =  money * count;
               while i >= j{
                   if dp[i as usize] == 0 {
                     dp[i as usize] = dp[ (i - j) as usize] + count;
                   } else {
                     dp[i as usize] = (dp[ (i - j) as usize] + count).min(dp[i as usize]);
                   }
                   count += 1;
                   j = money * count
               }
           }
       } 
    }
    dp[amount as usize] 
}
