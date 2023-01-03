use rand::Rng;

#[derive(Debug)]
pub struct WeightRandom {
    pub item: char,
    pub weight: i32,
}

impl WeightRandom {
    pub fn new(v: &Vec<(char, i32)>,m: usize) -> Vec< Self> {
    //v: input Vec [(item, weight), ...]
    //m: number of selected items
    //return: Vec [(item, weight), ...]
    let mut res = Vec::new();
    for sample in v {
        let wi = f64::from(sample.1);
        // rand 0-1
        let ui: f64 = rand::thread_rng().gen_range(0.0..1.0);
        let ki: f64 = ui.powf(1.0 / wi);

        if res.len() < m {
            res.push((ki, sample));
        } else if ki > res[0].0 {
            res.push((ki, sample));

            if res.len() > m {
                res.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                res.pop();
            }
        }
        // println!("{:?}", res);
    }
    // 取出第二位置的Vec
    let mut res2 = Vec::new();
    for i in res {
        res2.push( WeightRandom {
            item: i.1.0,
            weight: i.1.1,
        })
    }
    // println!("{:?}", res2);
    res2
    }
}