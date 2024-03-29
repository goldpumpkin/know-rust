use std::io::Read;
use std::mem::take;

use crate::service::CommandService;
use crate::storage::Storage;
use crate::*;

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => KvError::InvalidCommand(format!("{:?}", self)).into(),
        }
    }
}

impl CommandService for Hdel {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.del(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(error) => error.into(),
        }
    }
}

impl CommandService for Hmget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let mut result = Vec::new();

        for key in self.keys {
            match store.get(&self.table, key.as_str()) {
                Ok(Some(v)) => {
                    let keypair = Kvpair::new(key, v);
                    result.push(keypair);
                }
                Err(_) => {}
                _ => {}
            }
        }

        result.into()
    }
}

impl CommandService for Hexist {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.contains(&self.table, &self.key) {
            Ok(v) => v.into(),
            Err(error) => error.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::StatusCode;

    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into()], &[]);
    }

    #[test]
    fn hget_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);
    }

    #[test]
    fn hget_with_non_exist_key_should_return_404() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_error(res, 404, "Not found");
    }

    #[test]
    fn hgetall_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("score", "u1", 10.into()),
            CommandRequest::new_hset("score", "u2", 8.into()),
            CommandRequest::new_hset("score", "u3", 11.into()),
            CommandRequest::new_hset("score", "u1", 6.into()),
        ];
        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("u1", 6.into()),
            Kvpair::new("u2", 8.into()),
            Kvpair::new("u3", 11.into()),
        ];
        assert_res_ok(res, &[], pairs);
    }

    #[test]
    fn hdel_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("t1", "k1", "v1".into()),
            CommandRequest::new_hset("t1", "k2", "v2".into()),
        ];
        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd_del = CommandRequest::new_del("t1", "k1");
        let res = dispatch(cmd_del, &store);
        assert_res_ok(res, &["v1".into()], &[]);
    }

    #[test]
    fn hdel_should_work2() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "k1", "v1".into());
        dispatch(cmd, &store);

        let cmd_del = CommandRequest::new_del("t1", "k1");
        let res = dispatch(cmd_del, &store);
        assert_res_ok(res, &["v1".into()], &[]);
    }

    #[test]
    fn hmget_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("t", "k1", 10.into()),
            CommandRequest::new_hset("t", "k2", "v2".into()),
            CommandRequest::new_hset("t", "k3", "1".into()),
        ];

        for c in cmds {
            dispatch(c, &store);
        }

        let keys = vec!["k1".to_string(), "k2".to_string(), "k3".to_string()];
        let cmd = CommandRequest::new_hmget("t", keys);
        let res = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("k1", 10.into()),
            Kvpair::new("k2", "v2".into()),
            Kvpair::new("k3", "1".into()),
        ];
        assert_res_ok(res, &[], pairs);
    }

    #[test]
    fn hexist_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_exist("t", "k");
        let res = dispatch(cmd, &store);
        assert_res_error(res, StatusCode::NOT_FOUND.as_u16() as _, "")
    }
}
