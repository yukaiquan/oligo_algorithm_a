use oligo_algorithm_a::WeightRandom;

fn main() {
    // 新建一个Vec
    let v:Vec<(char, i32)> = vec![('a', 10), ('b', 20), ('c', 50), ('d', 100), ('e', 200)];
    // 调用算法
    // let res = a_res(&v, 2);
    let res = WeightRandom::new(&v, 2);
    println!("{:?}", res);
}


// a_res 算法
// fn a_res(v: &Vec<(char, i32)>, m: usize) -> Vec<(char, i32)> {
//     //v: input Vec [(item, weight), ...]
//     //m: number of selected items
//     //return: Vec [(item, weight), ...]
//     let mut res = Vec::new();
//     for sample in v {
//         let wi = f64::from(sample.1);
//         // rand 0-1
//         let ui: f64 = rand::thread_rng().gen_range(0.0..1.0);
//         let ki: f64 = ui.powf(1.0 / wi);

//         if res.len() < m {
//             res.push((ki, sample));
//         } else if ki > res[0].0 {
//             res.push((ki, sample));

//             if res.len() > m {
//                 res.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
//                 res.pop();
//             }
//         }
//         println!("{:?}", res);
//     }
//     // 取出第二位置的Vec
//     let mut res2 = Vec::new();
//     for i in res {
//         res2.push(*i.1);
//     }
//     res2
// }

// A-ExpJ 算法
