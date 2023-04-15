#[allow(unused)]

// 泛型默认是需要实现 Sized trait
struct Data<T> {}

fn process_data(data: Data<T>) {}

// 上面的 Data<T> 是省略了 Sized
struct DataSized<T: Sized> {}

fn process_data_sized(data: DataSized<T>) {}

// ?Sized 任意大小
pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

fn main() {}