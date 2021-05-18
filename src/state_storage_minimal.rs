use std::sync::Arc;
use std::sync::RwLock;
use state::Storage;

#[derive(Debug)]
pub enum SettingsError {
}

static SETTINGS_CONTAINER: Storage<RwLock<Vec<i32>>> = Storage::new();

pub fn handle() -> Result<Arc<Vec<i32>>, SettingsError> {
  let container_read = SETTINGS_CONTAINER.get().read().unwrap();
  Ok(Arc::new(container_read.clone()))
}
pub fn upsert(i: i32) -> Result<bool, SettingsError> {
  let container = SETTINGS_CONTAINER.try_get();
  match container {
    Some(_) => {
      let mut container_write = container.unwrap().write().unwrap();
      container_write.push(i);
    },
    None => {
      SETTINGS_CONTAINER.set(RwLock::new(vec![0, 1, i]));
    }
  }
  Ok(true)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn upsert_2() {
    let res = upsert(2);
    assert!(res.unwrap());
  }
  #[test]
  fn handle_3() {
    upsert(3).unwrap();
    let _res = handle().unwrap();
    assert_eq!(*_res, vec![0, 1, 2, 3]);
  }
  #[test]
  fn handle_456() {
    upsert(4).unwrap();
    upsert(5).unwrap();
    upsert(6).unwrap();
    let _res = handle().unwrap();
    assert_eq!(*_res, vec![0, 1, 4, 5, 6]);
  }
  #[test]
  fn handle_789() {
    upsert(7).unwrap();
    upsert(8).unwrap();
    upsert(9).unwrap();
    let _res = handle().unwrap();
    assert_eq!(*_res, vec![0, 1, 7, 8, 9]);
  }
}

