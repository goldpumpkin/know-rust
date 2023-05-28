mod memory;

pub use memory::MemTable;

use crate::{KvError, Kvpair, Value};

pub trait Storage {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个 HashTable 里面设置一个 key 的 value， 返回旧的 value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 从查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    /// 遍历 HashTable， 返回所有 kv pair - bad trait
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// 遍历 HashTable， 返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}
