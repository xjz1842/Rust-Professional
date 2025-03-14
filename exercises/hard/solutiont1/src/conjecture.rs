
pub fn goldbach_conjecture() -> String {
    let mut prime_vec: Vec<u64> = Vec::new();  // 质数缓存列表（用于快速查找）
    let mut result_vec: Vec<u64> = Vec::new(); // 结果集合（存储符合条件的反例）
    let mut temp: u64 = 3;  // 当前检测的数字（从最小奇数开始）

    prime_vec.push(2);  // 初始化第一个质数

    // 主循环：直到找到两个符合条件的数字
    while result_vec.len() < 2 {
        if is_prime(temp) {
            // 如果是质数则加入质数列表
            prime_vec.push(temp);
        } else {
            // 非质数时进行表达式验证
            let mut index: usize = 0;
            let mut flag: bool = true;  // 标记是否找到合法组合
            
            // 遍历所有已知小干temp的质数
            while index < prime_vec.len() && temp > prime_vec[index] {
                let mut tmp: u64 = 1;  // 平方数系数
                
                // 寻找满足 temp = prime + 2*(tmp^2) 的组合
                // 数学等价性检测：tmp^2 = (temp - prime)/2
                while tmp * tmp * 2 + prime_vec[index] < temp {
                    tmp += 1;
                }
                
                // 找到合法组合则标记并跳出
                if tmp * tmp * 2 + prime_vec[index] == temp {
                    flag = false;
                    break;
                }
                index += 1;
            }

            // 未找到任何合法组合则记录结果
            if flag {
                result_vec.push(temp);
            }
        }

        temp += 2;  // 仅检测奇数（偶数可以直接用哥德巴赫原猜想）
    }

    return format!("{},{}", result_vec[0], result_vec[1]);
}

/// 基础质数判断函数（有待优化）
/// 参数：number - 需要判断的u64整数
/// 返回：true表示是质数，false表示合数
fn is_prime(number: u64) -> bool {
    // 优化点：应改为sqrt(number)+1以降低计算量
    let stop: u64 = number / 2 + 1;
    let mut temp: u64 = 2;
    
    // 遍历所有可能的因数
    while temp < stop {
        if number % temp == 0 {
            return false;  // 发现因数直接返回
        }
        temp += 1;
    }
    return true;  // 无因数则为质数
}