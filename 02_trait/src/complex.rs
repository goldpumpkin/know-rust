use std::ops::Add;

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

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.real + rhs.real;
        let i = self.imagine + rhs.imagine;
        Complex::new(r, i)
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.real + rhs.real;
        let i = self.imagine + rhs.imagine;
        Complex::new(r, i)
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Complex::new(real, self.imagine)
    }
}


fn main() {
    let c1 = Complex::new(1.0, 2f64);
    let c2 = Complex::new(3 as f64, 3.3);
    println!("result: {:?}", &c1 + &c2);

    let result = c1 + c2;
    println!("result: {:?}", result);

    // 所有权转移了，则不能进行编译
    // println!("result: {:?}", c1 + c2);
}