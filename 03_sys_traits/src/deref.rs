use std::ops::{Deref, DerefMut};

// 这个代码定义了一个名为 Buffer 的结构体，它含有一个泛型参数 T。这个结构体内部使用了一个 Vec<T> 来存储数据，也就是说创建一个 Buffer<T> 的实例就相当于创建了一个存储类型为 T 的元素的可变长度的数组。
// 因此，你可以通过 Buffer<T> 的实例来使用向量 (Vec<T>) 所提供的方法和功能，例如添加元素，删除元素，获取元素等。同时，由于 T 是泛型参数，所以可以在使用 Buffer 结构体时指定不同的类型参数来创建不同类型的缓冲区，以满足不同的需求。
#[derive(Debug)]
struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    pub fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut buf = Buffer::new([5, 2, 3, 4]);
    println!("buf:{:?}", buf);

    buf.sort_unstable();
    println!("sorted buf:{:?}", buf)
}