use chashmap::CHashMap;

#[derive(Debug, Clone)]
pub struct KvElement {
    pub data: Vec<u8>,
    // pub mime_type: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
    // pub expire_at: DateTime<Utc>,
    // pub update_count: i32,
    pub locked: bool,
}

pub struct KvStore {
  container: CHashMap<String, KvElement>,
  // cipher: Option<Cipher>,
}

// pub struct Cipher {
//   priv_key: [u8; 24],
//   iv: [u8; 16],
// }


impl KvStore {
  pub fn new() -> KvStore {
      // TODO: prepare looped persistence
      let mut kv = KvStore {
          container: CHashMap::new()
      };

      // if let Some(c) = cipher {
      //     let (mut priv_key, mut iv) = ([0u8; 24], [0u8; 16]);
      //     priv_key[..24].copy_from_slice(&hex::decode(c[0]).unwrap());
      //     iv[..16].copy_from_slice(&hex::decode(c[1]).unwrap());
      //     kv.cipher = Some(Cipher { priv_key, iv });
      // }

      kv
  }

  pub fn set(&self, key: String, mut value: Vec<u8>) -> Option<KvElement> {
      // TODO: prepare iterative persistence
      // if let Some(c) = &self.cipher {
      //     let cipher = SerpentCbc::new_var(&c.priv_key, &c.iv).unwrap();
      //     value = cipher.encrypt_vec(&value);
      // }
      // let mime_type = match mime {
      //     Some(gived_mimetype) => gived_mimetype,
      //     None => tree_magic::from_u8(value.as_ref()).to_string(),
      // };
      match &mut self.container.get_mut(&key) {
          Some(kv_element) => {
              if !kv_element.locked {
                  kv_element.data = value;
              }
            //   kv_element.updated_at = Utc::now();
            //   kv_element.update_count = kv_element.update_count + 1;
              Some(kv_element.to_owned())
          }
          None => {
              let kv_element = KvElement {
                  data: value,
                //   created_at: Utc::now(),
                //   updated_at: Utc::now(),
                //   expire_at: Utc::now(),
                //   update_count: 1,
                  locked: false,
              };
              self.container.insert(key, kv_element)
          }
      }
  }

  pub fn get(&self, key: String) -> Option<KvElement> {
      match self.container.get(&key) {
          Some(value) => {
              let mut cloned_value = value.clone();

              // if let Some(c) = &self.cipher {
              //     let cipher = SerpentCbc::new_var(&c.priv_key, &c.iv).unwrap();
              //     cloned_value.data = cipher.decrypt_vec(&value.data).unwrap();
              // }
              Some(cloned_value)
          }
          None => None,
      }
  }

  pub fn switch_lock(&self, key: String, to_lock: bool) -> bool {
      match &mut self.container.get_mut(&key) {
          Some(kv_element) => {
              if kv_element.locked == to_lock {
                  return false;
              }
              kv_element.locked = to_lock;
              true
          }
          None => false,
      }
  }

  pub fn drop(&self, key: String) {
      self.container.remove(&key);
  }
}

