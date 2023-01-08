use oligo_algorithm_a::WeightRandom;
use std::collections::HashMap;
use std::time::SystemTime;

fn main() {
    // 新建一个Vec
    let v: Vec<(char, isize)> = vec![('a', 10), ('b', 200), ('c', 50), ('d', 100), ('e', 20)];
    // 默认算法
    println!("默认算法");
    println!("a_res 选择一个");
    let sy_time = SystemTime::now();
    test_10w(&v, true, 1);
    println!("{:?}", SystemTime::now().duration_since(sy_time).unwrap());
    println!("a_res 选择两个");
    test_10w(&v, true, 2);
    println!("a_res 选择三个");
    test_10w(&v, true, 3);
    println!("a_res 选择四个");
    test_10w(&v, true, 4);
    println!("a_res 选择五个");
    test_10w(&v, true, 5);
    // a_expj算法
    println!("a_expj算法");
    println!("a_expj 选择一个");
    let sexpj_time = SystemTime::now();
    test_10w(&v, false, 1);
    println!(
        "{:?}",
        SystemTime::now().duration_since(sexpj_time).unwrap()
    );
    println!("a_expj 选择两个");
    test_10w(&v, false, 2);
    println!("a_expj 选择三个");
    test_10w(&v, false, 3);
    println!("a_expj 选择四个");
    test_10w(&v, false, 4);
    println!("a_expj 选择五个");
    test_10w(&v, false, 5);
}

fn test_10w(v: &Vec<(char, isize)>, a_res: bool, m: usize) {
    if a_res {
        let mut state = HashMap::new();
        let mut result = HashMap::new();
        for _i in 0..100000 {
            let res = WeightRandom::new(&v, m);
            for i in res {
                let count = state.entry(i.item).or_insert(0);
                *count += 1;
            }
        }
        let total = state.get(&'a').unwrap();
        for i in &state {
            let percent = *i.1 as f64 / *total as f64;
            result.insert(i.0, percent);
        }
        println!("{:?}", result);
    } else {
        let mut state = HashMap::new();
        let mut result = HashMap::new();
        for _i in 0..100000 {
            let res = WeightRandom::a_expj(&v, m);
            for i in res {
                let count = state.entry(i.item).or_insert(0);
                *count += 1;
            }
        }
        let total = state.get(&'a').unwrap();
        for i in &state {
            let percent = *i.1 as f64 / *total as f64;
            result.insert(i.0, percent);
        }
        println!("{:?}", result);
    }
}
