use rand::Rng;

#[derive(Debug)]
pub struct WeightRandom {
    pub item: char,
    pub weight: isize,
}

impl WeightRandom {
    // a_res 算法
    pub fn new(v: &Vec<(char, isize)>, m: usize) -> Vec<Self> {
        //v: input Vec [(item, weight), ...]
        //m: number of selected items
        //return: Vec [(item, weight), ...]
        // let mut res = Vec::new();
        let mut heap = MinHeap::new();
        for sample in v {
            let wi = sample.1 as f64;
            // rand 0-1
            let ui: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let ki: f64 = ui.powf(1.0 / wi);

            if heap.payload.len() < m {
                heap.push_tail(ki, sample);
            } else if ki > heap.payload[0] {
                heap.push_tail(ki, sample);

                if heap.payload.len() > m {
                    heap.pop_head();
                }
            }

            // println!("{:?}", heap);
        }
        // 排序
        // 取出第二位置的Vec
        let mut res2 = Vec::new();
        for i in heap.input_name {
            res2.push(WeightRandom {
                item: i.0,
                weight: i.1,
            });
        }
        res2
    }
    // a_expj 算法
    pub fn a_expj(v: &Vec<(char, isize)>, m: usize) -> Vec<Self> {
        //v: input Vec [(item, weight), ...]
        //m: number of selected items
        //return: Vec [(item, weight), ...]
        let mut heap = MinHeap::new();
        let mut x_w = 0 as f64;
        let mut t_w: f64 = 0 as f64;
        let mut w_acc = 0 as f64;
        for sample in v {
            if heap.payload.len() < m {
                let wi = sample.1 as f64;
                let ui: f64 = rand::thread_rng().gen_range(0.0..1.0);
                let ki: f64 = ui.powf(1.0 / wi);
                heap.push_tail(ki, sample);
                continue;
            }
            if w_acc == 0 as f64 {
                t_w = heap.payload[0];
                let r: f64 = rand::thread_rng().gen_range(0.0..1.0);
                // xW = math.log(r)/math.log(Tw);
                x_w = r.log(2.0) / t_w.log(2.0);
            }
            let wi = sample.1 as f64;
            if w_acc + wi < x_w {
                w_acc += wi;
                continue;
            } else {
                w_acc = 0 as f64;
            }
            let tw = t_w.powf(wi);
            let r2 = rand::thread_rng().gen_range(tw..1.0);
            let ki = r2.powf(1.0 / wi);
            heap.pop_head();
            heap.push_tail(ki, sample);
        }
        // 排序
        // 取出第二位置的Vec
        let mut res2 = Vec::new();
        for i in heap.input_name {
            res2.push(WeightRandom {
                item: i.0,
                weight: i.1,
            });
        }
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

#[derive(Debug)]
pub struct MinHeap<T>
where
    T: std::cmp::PartialOrd,
{
    payload: Vec<T>,
    input_name: Vec<(char, isize)>,
}
#[derive(Debug)]
pub struct Output<T>
where
    T: std::cmp::PartialOrd,
{
    pub item: Option<(char, isize)>,
    pub weight: Option<T>,
}

//为MinHeap绑定方法
impl<T> MinHeap<T>
where
    T: std::cmp::PartialOrd,
{
    pub fn new() -> MinHeap<T> {
        MinHeap {
            payload: Vec::new(),
            input_name: Vec::new(),
        }
    }
    fn length(&self) -> usize {
        self.payload.len()
    }
    fn less_element(&mut self, a: usize, b: usize) -> bool {
        //比较两个索引位置的元素大小 左边小于右边返回true
        self.payload[a] < self.payload[b]
    }
    fn swap_element(&mut self, a: usize, b: usize) {
        //交换两个索引位置的元素
        self.payload.swap(a, b);
        self.input_name.swap(a, b);
    }
    fn shiftup(&mut self, par: usize, size: usize) {
        let lchild = left_child!(par);
        let rchild = right_child!(par);
        // println!("par:{},lchild:{},rchild:{}", par, lchild, rchild);
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
        // println!("par:{},child:{}", par, child);
        if self.less_element(child, par) {
            // 这里没有写 0 <= par ,因为 par为uszie(默认>=0)
            self.swap_element(child, par);
            self.shiftdown(par);
        }
    }
    //加入元素
    pub fn push_tail(&mut self, element: T, element2: &(char, isize)) {
        self.payload.push(element);
        self.input_name.push(*element2);
        self.shiftdown(self.payload.len() - 1);
    }
    //删除最小的元素
    pub fn pop_head(&mut self) -> Output<T> {
        // let res = self.payload[0];
        self.swap_element(0, self.payload.len() - 1);
        let res = self.payload.pop();
        let res2 = self.input_name.pop();
        self.shiftup(0, self.length());
        // 两个数组合并为一个元组数组
        Output {
            item: res2,
            weight: res,
        }
    }
}
