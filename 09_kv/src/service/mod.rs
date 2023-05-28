use std::sync::Arc;

use tracing::debug;

use crate::command_request::RequestData;
use crate::storage::Storage;
use crate::{CommandRequest, CommandResponse, KvError, Kvpair, MemTable, Value};

mod command_service;

/// 对 Command 的处理抽象
pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

/// Service 数据结构
pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Service 内部数据结构
pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        // TODO: 发送 on_received 事件
        let res = dispatch(cmd, &self.inner.store);
        debug!("Executed response: {:?}", res);

        res
    }
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hgetall(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        None => KvError::InvalidCommand("Request has no data".into()).into(),
        _ => KvError::Internal("Not implemented".into()).into(),
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::{MemTable, Value};

    #[test]
    fn service_should_works() {
        let service = Service::new(MemTable::default());

        let cloned = service.clone();

        let handle = thread::spawn(move || {
            let cmd = CommandRequest::new_hset("t1", "k1", "v1".into());
            let res = cloned.execute(cmd);
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
pub fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

#[cfg(test)]
pub fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}
