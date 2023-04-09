use std::ops::Add;

// 实现参数传递时，值会复制
// #[derive(Debug, Copy, Clone)]
#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self {
            real,
            imagine,
        }
    }
}

// 实现 Complex 和 Complex 类型相加
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.real + rhs.real;
        let i = self.imagine + rhs.imagine;
        Complex::new(r, i)
    }
}

// 实现所有权不转移
impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.real + rhs.real;
        let i = self.imagine + rhs.imagine;
        Complex::new(r, i)
    }
}

//  实现 Complex 可以和 f64 类型相加
impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Complex::new(real, self.imagine)
    }
}

// 实现 Complex 可以和 $str 类型相加
impl Add<&str> for &Complex {
    type Output = Complex;

    fn add(self, rhs: &str) -> Self::Output {
        // 注意转换
        let c: f64 = rhs.parse().unwrap();
        let c1 = c + self.real;
        Complex::new(c1, self.imagine)
    }
}


fn main() {
    let c1 = Complex::new(1.0, 2f64);
    let c2 = Complex::new(3 as f64, 3.3);
    println!("result-1: {:?}", &c1 + &c2);

    let add_f64_num = &c1 + 2f64;
    println!("result-2: {:?}", add_f64_num);

    let add_str_num = &c1 + "3";
    println!("result-3: {:?}", add_str_num);

    let result = c1 + c2;
    println!("result-4: {:?}", result);


    // 所有权转移了，则不能进行编译
    // println!("result: {:?}", c1 + c2);
}