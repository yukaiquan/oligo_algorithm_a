// use oligo_algorithm_a::WeightRandom;
use std::collections::HashMap;
use std::fs;
// use std::time::SystemTime;

fn main() {
    // 需要的总的数量
    let mut need_num: HashMap<String, usize> = HashMap::new();
    need_num.insert("chr1A".to_string(), 100000);
    // 读取文件
    let v = fs::read_to_string("chr1A_test.bed").unwrap();
    // 逐行读取
    let mut oligo: Vec<(String, usize, usize, String)> = v
        .lines()
        .map(|x| {
            let mut x = x.split_whitespace();
            let chrom = x.next().unwrap().to_string();
            let start = x.next().unwrap().parse::<usize>().unwrap();
            let end = x.next().unwrap().parse::<usize>().unwrap();
            let seq = x.next().unwrap().to_string();
            (chrom, start, end, seq)
        })
        .collect();
    // 打印一行
    // println!("{:?}", v[0]);
    // 对第一列进行分组 求第二列最大最小值
    let (chr_min_pos, chr_max_pos) = cacu_max_min_chr_pos(&oligo);
    // println!("{:?}", chr_min_pos);
    // println!("{:?}", chr_max_pos);
    let chr_bin = chr_bin_split(&chr_min_pos, &chr_max_pos, &need_num);
    // 根据染色体长度和需要的数量进行切分bin生成空的hashmap
    let mut chr_bin_oligo_num = HashMap::new();
    // 存储区间和oligo对应关系
    let mut chr_bin_oligo = HashMap::new();
    for (key, value) in chr_max_pos.iter() {
        let bin_len = need_num.get(key).unwrap();
        let bin_num = value / bin_len;
        for i in 0..bin_num + 1 {
            let bin_start = i * bin_len;
            let mut bin_end = (i + 1) * bin_len;
            if bin_end > *value {
                bin_end = *value;
            }
            let key = format!("{}_{}_{}", key, bin_start, bin_end);
            chr_bin_oligo_num.insert(key.clone(), 0);
            chr_bin_oligo.insert(key, vec![]);
        }
    }
    // 遍历oligo快速将其分配到bin中
    for i in oligo.iter() {
        // 快速根据usize生成key
        let bin_len = need_num
            .get(&i.0.to_string())
            .expect("chromosome not found");
        let max_pos = chr_max_pos
            .get(&i.0.to_string())
            .expect("chromosome not found");
        let (bin_start, bin_end) = len_size_generator_bin(i.1, *bin_len, *max_pos);
        let key = format!("{}_{}_{}", i.0, bin_start, bin_end);
        // 判断key是否存在
        // 如果存在，加1
        if let Some(num) = chr_bin_oligo_num.get_mut(&key) {
            *num += 1;
            chr_bin_oligo.entry(key).or_insert(vec![]).push(i);
        } else {
            // 如果不存在，设置为1
            eprintln!("{} not found", key)
        }
    }
    // println!(
    //     "{:?}:{:?}",
    //     chr_bin_oligo_num.get("chr1A_525800000_525896434"),
    //     chr_bin_oligo.get("chr1A_525800000_525896434")
    // );
    let mut oligo_result_vec = vec![];
    for i in oligo.iter() {
        let bin_len = need_num
            .get(&i.0.to_string())
            .expect("chromosome not found");
        let max_pos = chr_max_pos
            .get(&i.0.to_string())
            .expect("chromosome not found");
        let (bin_start, bin_end) = len_size_generator_bin(i.1, *bin_len, *max_pos);
        let key = format!("{}_{}_{}", i.0, bin_start, bin_end);
        let num = chr_bin_oligo_num.get(&key).unwrap();
        let chr = i.0.to_string();
        let start = i.1;
        let end = i.2;
        let seq = i.3.to_string();
        if num == &1 {
            // println!("{}", key);
            let weight = 10;
            let left_bin_num = 5;
            let right_bin_num = 5;
            let (left_bin_vec, right_bin_vec) = generator_left_right_bin(
                &key,
                &need_num,
                &chr_max_pos,
                left_bin_num,
                right_bin_num,
            );
            // println!("{:?}", left_bin_vec);
            // println!("{:?}", right_bin_vec);
            let mut left_num = 0;
            let mut right_num = 0;
            for j in left_bin_vec.iter() {
                // 查不到的时候设置为-1代表bin已经越界 当然上方已经解决bin越界的问题
                let num = chr_bin_oligo_num.get(j).unwrap_or(&-1);
                // 都为0的时候，权重为1
                if num > &0 || num == &-1 {
                    left_num += 1;
                }
            }
            for j in right_bin_vec.iter() {
                // 查不到的时候设置为-1
                let num = chr_bin_oligo_num.get(j).unwrap_or(&-1);
                // 都为0的时候，权重为1
                if num > &0 || num == &-1 {
                    right_num += 1;
                }
            }
            // 只要左右两边有一个不为0，权重为10
            if left_num > 0 || right_num > 0 {
                // println!(
                //     "{}:{}:{}:{}:{}",
                //     key, left_num, right_num, left_bin_num, right_bin_num
                // );
                oligo_result_vec.push((chr, start, end, seq, weight, true));
            } else {
                oligo_result_vec.push((chr, start, end, seq, 1, false));
            }
        } else {
            // 查看左右两边的bin是否为1 如果为1，根据位置给目前的区间距离一个bin的位置加权重
            // 如果左右两边的bin不为1，则给目前区间最中间的位置加权重
            let weight = 10;
            let left_bin_num = 1;
            let right_bin_num = 1;
            let (left_bin_vec, right_bin_vec) = generator_left_right_bin(
                &key,
                &need_num,
                &chr_max_pos,
                left_bin_num,
                right_bin_num,
            );
            // println!("{:?}", left_bin_vec);
            // println!("{:?}", right_bin_vec);
            let mut left_num = 0;
            let mut right_num = 0;
            for j in left_bin_vec.iter() {
                // 查不到的时候设置为-1代表bin已经越界 当然上方已经解决bin越界的问题
                let num = chr_bin_oligo_num.get(j).unwrap_or(&-1);
                // 都为0的时候，权重为1
                if num > &0 || num == &-1 {
                    left_num += 1;
                }
            }
            for j in right_bin_vec.iter() {
                // 查不到的时候设置为-1
                let num = chr_bin_oligo_num.get(j).unwrap_or(&-1);
                // 都为0的时候，权重为1
                if num > &0 || num == &-1 {
                    right_num += 1;
                }
            }
            // 只要左右两边有一个不为0，权重按照距离加权
        }
    }

    // // 新建一个Vec
    // let v: Vec<(char, isize)> = vec![('a', 10), ('b', 200), ('c', 50), ('d', 100), ('e', 20)];
    // // 默认算法
    // println!("默认算法");
    // println!("a_res 选择一个");
    // let sy_time = SystemTime::now();
    // test_10w(&v, true, 1);
    // println!("{:?}", SystemTime::now().duration_since(sy_time).unwrap());
    // println!("a_res 选择两个");
    // test_10w(&v, true, 2);
    // println!("a_res 选择三个");
    // test_10w(&v, true, 3);
    // println!("a_res 选择四个");
    // test_10w(&v, true, 4);
    // println!("a_res 选择五个");
    // test_10w(&v, true, 5);
    // // a_expj算法
    // println!("a_expj算法");
    // println!("a_expj 选择一个");
    // let sexpj_time = SystemTime::now();
    // test_10w(&v, false, 1);
    // println!(
    //     "{:?}",
    //     SystemTime::now().duration_since(sexpj_time).unwrap()
    // );
    // println!("a_expj 选择两个");
    // test_10w(&v, false, 2);
    // println!("a_expj 选择三个");
    // test_10w(&v, false, 3);
    // println!("a_expj 选择四个");
    // test_10w(&v, false, 4);
    // println!("a_expj 选择五个");
    // test_10w(&v, false, 5);
}

fn generator_left_right_bin(
    key: &str,
    need_num: &HashMap<String, usize>,
    chr_max_pos: &HashMap<String, usize>,
    left_num: usize,
    right_num: usize,
) -> (Vec<String>, Vec<String>) {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();
    let bin_vec = key.split("_").collect::<Vec<&str>>();
    let chr_name = bin_vec[0];
    for i in 1..left_num + 1 {
        let left_bin_start =
            bin_vec[1].parse::<usize>().unwrap() - need_num.get(chr_name).unwrap() * i;
        if left_bin_start < 0 {
            break;
        }
        let left_bin_end =
            bin_vec[1].parse::<usize>().unwrap() - need_num.get(chr_name).unwrap() * (i - 1);
        let left_key = format!("{}_{}_{}", chr_name, left_bin_start, left_bin_end);
        left_vec.push(left_key);
    }
    for i in 1..right_num + 1 {
        let right_bin_start =
            bin_vec[2].parse::<usize>().unwrap() + need_num.get(chr_name).unwrap() * (i - 1);
        let right_bin_end =
            bin_vec[2].parse::<usize>().unwrap() + need_num.get(chr_name).unwrap() * i;
        if right_bin_start > *chr_max_pos.get(chr_name).unwrap()
            && right_bin_end > *chr_max_pos.get(chr_name).unwrap()
        {
            break;
        } else if right_bin_start < *chr_max_pos.get(chr_name).unwrap()
            && right_bin_end > *chr_max_pos.get(chr_name).unwrap()
        {
            let right_key = format!(
                "{}_{}_{}",
                chr_name,
                right_bin_start,
                chr_max_pos.get(chr_name).unwrap()
            );
            right_vec.push(right_key);
            break;
        } else if right_bin_start >= *chr_max_pos.get(chr_name).unwrap()
            && right_bin_end > *chr_max_pos.get(chr_name).unwrap()
        {
            // 不返回值
            break;
        }
        let right_key = format!("{}_{}_{}", chr_name, right_bin_start, right_bin_end);
        right_vec.push(right_key);
    }
    (left_vec, right_vec)
}

fn len_size_generator_bin(pos: usize, len: usize, max: usize) -> (usize, usize) {
    // '''
    // 根据position和bin的长度最大值，计算bin的起始和结束位置
    // input: pos:usize,len:usize,max:usize
    // output: (usize,usize)
    // '''
    let bin = pos / len * len;
    let bin_start = bin;
    let mut bin_end = bin + len;
    if bin_end > max {
        bin_end = max
    }
    (bin_start, bin_end)
}

fn chr_bin_split(
    min_map: &HashMap<String, usize>,
    max_map: &HashMap<String, usize>,
    bin_map: &HashMap<String, usize>,
) -> HashMap<String, Vec<(usize, usize, usize)>> {
    // '''
    // 利用数量切分bin
    // 遍历hashmap
    // input: min_map:HashMap<String,usize>,max_map:HashMap<String,usize>,bin_map:HashMap<String,usize>
    // output: HashMap<String,Vec<(usize,usize,usize)>>
    // Vec<(usize,usize,usize) 为bin的起始位置和结束位置 以及需要oligo的数量
    // '''
    let mut chr_bin = HashMap::new();
    for (k, v) in bin_map {
        let max_pos = max_map.get(k).unwrap();
        let _min_pos = min_map.get(k).unwrap();
        // 产生长度为max_pos长度为v的bin
        let bin = bin_split(*max_pos, *v);
        // 存入hashmap
        // 判断是否key存在
        if chr_bin.contains_key(k) {
            // 存在提示错误
            eprintln!("{} chr is duplicate", k);
        } else {
            // 不存在
            chr_bin.insert(k.clone(), bin);
        }
    }
    chr_bin
}

fn bin_split(max_pos: usize, bin: usize) -> Vec<(usize, usize, usize)> {
    // '''
    // input: max_pos: usize, bin: usize
    // output: Vec<(usize,usize)>
    // '''
    let mut bin_vec = vec![];
    let bin_size = max_pos / bin;
    for i in 0..bin_size {
        let start = i * bin;
        let mut end = (i + 1) * bin;
        if end > max_pos {
            end = max_pos;
        }
        bin_vec.push((start, end, 1));
    }
    bin_vec
}

fn cacu_max_min_chr_pos(
    v: &Vec<(String, usize, usize, String)>,
) -> (HashMap<String, usize>, HashMap<String, usize>) {
    // '''
    // input: v: Vec<(String, isize, isize, String)>
    // output: (HashMap<String, isize>, HashMap<String, isize>)
    // '''
    // 对第一列进行分组 求第二列最大最小值
    let mut chr_min_pos = HashMap::new();
    let mut chr_max_pos = HashMap::new();
    for i in v {
        let chrom = i.0.clone();
        let chrom2 = chrom.clone();
        let min_pos = i.1;
        let max_pos = i.1;
        if chr_min_pos.contains_key(&chrom) {
            // 更改v2中的值
            let hash_value = chr_min_pos.get_mut(&chrom).unwrap();
            if *hash_value > min_pos {
                *hash_value = min_pos;
            }
        } else {
            chr_min_pos.insert(chrom, min_pos);
        }
        if chr_max_pos.contains_key(&chrom2) {
            // 更改v2中的值
            let hash_value = chr_max_pos.get_mut(&chrom2).unwrap();
            if *hash_value < max_pos {
                *hash_value = max_pos;
            }
        } else {
            chr_max_pos.insert(chrom2, max_pos);
        }
    }
    (chr_min_pos, chr_max_pos)
}

// fn test_10w(v: &Vec<(char, isize)>, a_res: bool, m: usize) {
//     if a_res {
//         let mut state = HashMap::new();
//         let mut result = HashMap::new();
//         for _i in 0..100000 {
//             let res = WeightRandom::new(&v, m);
//             for i in res {
//                 let count = state.entry(i.item).or_insert(0);
//                 *count += 1;
//             }
//         }
//         let total = state.get(&'a').unwrap();
//         for i in &state {
//             let percent = *i.1 as f64 / *total as f64;
//             result.insert(i.0, percent);
//         }
//         println!("{:?}", result);
//     } else {
//         let mut state = HashMap::new();
//         let mut result = HashMap::new();
//         for _i in 0..100000 {
//             let res = WeightRandom::a_expj(&v, m);
//             for i in res {
//                 let count = state.entry(i.item).or_insert(0);
//                 *count += 1;
//             }
//         }
//         let total = state.get(&'a').unwrap();
//         for i in &state {
//             let percent = *i.1 as f64 / *total as f64;
//             result.insert(i.0, percent);
//         }
//         println!("{:?}", result);
//     }
// }
