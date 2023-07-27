use std::collections::HashMap;

// 简单的匹配宏
macro_rules! hello {
    () => { println!("hello welcome"); };
    ($name: expr) => { println!("hello {}", $name); };
    ($name1: expr, $name2: expr) => { println!("hello {}, hello {}", $name1, $name2); };
}

// 这个宏的作用是遍历数组
macro_rules! each {
    ($arr: expr, $func: expr) => {
        {
            let mut idx = 0;
            while idx < $arr.len() {
                $func(&$arr[idx]);      // 取引用，不要所有权
                idx += 1;
            }
            println!();
        }
    };
}

fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    each!(arr, |num| {
        print!("{} ", num);
    });

    let arr: Vec<&str> = vec!["abc", "xyz", "xxx"];
    each!(arr, |num| {
        print!("{} ", num);
    });
}

