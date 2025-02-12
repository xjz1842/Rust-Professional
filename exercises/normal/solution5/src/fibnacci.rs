pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold == 0 {
        return 0;
    }
    // 初始化斐波那契数列前两项
    let (mut a, mut b) = (0, 1);
    let mut sum = 0;
    // 生成斐波那契数列，直到超过 n
    while a <= threshold {
        // 检查当前项是否为奇数
        if a % 2 != 0 {
            sum += a;
        }
        // 更新下一项：a ← b, b ← a + b
        let next = a + b;
        a = b;
        b = next;
    }
    sum
}
