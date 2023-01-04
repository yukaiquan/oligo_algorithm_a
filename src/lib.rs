use rand::Rng;

#[derive(Debug)]
pub struct WeightRandom {
    pub item: char,
    pub weight: i32,
}

impl WeightRandom {
    // a_res 算法
    pub fn new(v: &Vec<(char, i32)>, m: usize) -> Vec<Self> {
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
                    res.pop();
                }
            }
            // println!("{:?}", res);
        }
        // 排序
        // 取出第二位置的Vec
        let mut res2 = Vec::new();
        for i in res {
            res2.push(WeightRandom {
                item: i.1 .0,
                weight: i.1 .1,
            })
        }
        // println!("{:?}", res2);
        res2
    }
}

//MinHeap
macro_rules! parent {
    ($child:ident) => {
        match $child {
            0 => 0,
            _ => ($child - 1) / 2,
        }
    };
}
macro_rules! left_child {
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}
macro_rules! right_child {
    ($parent:ident) => {
        ($parent + 1) << 1
    };
}
pub struct MinHeap<T>
where
    T: std::cmp::PartialOrd,
{
    payload: Vec<T>,
}
//为MinHeap绑定方法
impl<T> MinHeap<T>
where
    T: std::cmp::PartialOrd,
{
    pub fn new() -> MinHeap<T> {
        MinHeap {
            payload: Vec::new(),
        }
    }
    fn length(&self) -> usize {
        self.payload.len()
    }
    fn less_element(&mut self, a: usize, b: usize) -> bool {
        self.payload[a] < self.payload[b]
    }
    fn swap_element(&mut self, a: usize, b: usize) {
        self.payload.swap(a, b);
    }
    fn shiftup(&mut self, par: usize, size: usize) {
        let lchild = left_child!(par);
        let rchild = right_child!(par);
        let mut min = par;
        if lchild < size && self.less_element(lchild, min) {
            min = lchild;
        }
        if rchild < size && self.less_element(rchild, min) {
            min = rchild;
        }
        if min != par {
            self.swap_element(par, min);
            self.shiftup(min, size);
        }
    }

    //建立最小堆
    pub fn build_heap(&mut self) {
        let l = self.payload.len();
        let l_half = l >> 1;
        let mut i = l_half + 1;
        while i >= 1 {
            i -= 1;
            self.shiftup(i, l);
        }
    }
    //堆排序 值大到小
    pub fn heapsort(&mut self) {
        let mut running_size = self.length() - 1;
        while running_size > 0 {
            self.swap_element(0, running_size);
            self.shiftup(0, running_size);
            running_size -= 1;
        }
    }

    //大下标 --> 小下标
    fn shiftdown(&mut self, child: usize) {
        let par = parent!(child);
        if self.less_element(child, par) {
            // 这里没有写 0 <= par ,因为 par为uszie(默认>=0)
            self.swap_element(child, par);
            self.shiftdown(par);
        }
    }
    //加入元素
    pub fn push_tail(&mut self, element: T) {
        self.payload.push(element);
        self.shiftdown(self.payload.len() - 1);
    }
    //删除最小的元素
    pub fn pop_head(&mut self) -> Option<T> {
        // let res = self.payload[0];
        self.swap_element(0, self.payload.len() - 1);
        let res = self.payload.pop();
        self.shiftup(0, self.length());
        res
    }
}
