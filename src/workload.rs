use rand::Rng;
use rand::distributions::{Distribution, Uniform, Alphanumeric};
use rand::rngs::ThreadRng;

#[derive(Debug, Copy, Clone)]
pub struct Ratios {
  pub put: f32,
  pub get: f32,
  pub delete: f32,
}

pub struct Operation {
  pub category: i32,
  pub key: String,
  pub body: String
}

pub struct Workload {
  config: Ratios,
  operation_count: i64,
  resource_count: i64,
  body_length: i64,
  current_index: i64,
  rng : ThreadRng,
  operation_distrib : Uniform<i32>, 
  resource_id_distrib: Uniform<i64>,
  length_distrib : Uniform<i64>,
  random_str : String,
}


impl Workload {
  pub fn new(conf: Ratios, op_cnt: i64, res_cnt: i64, body_len: i64) -> Workload {
    Workload { config: conf, operation_count: op_cnt, resource_count: res_cnt, body_length: body_len, current_index: 0, rng: rand::thread_rng(), operation_distrib: Uniform::from(0..100), resource_id_distrib: Uniform::from(0..res_cnt), length_distrib: Uniform::from(0..body_len), random_str: rand::thread_rng().sample_iter(&Alphanumeric).take(body_len as usize).map(char::from).collect()} 
  }

  fn generate_op(&mut self) -> i32 {
    let n1 = self.operation_distrib.sample(&mut self.rng);
    let pos = (n1 as f32)/100.0;
    if pos < self.config.put {
      0
    } else if pos < self.config.put + self.config.get {
      1
    } else {
      2
    }
  }

  fn gen_random_key(&mut self) -> String {
    let index = self.resource_id_distrib.sample(&mut self.rng);
    let resouce_str = format!("{}{}{}", "resource", "_", index);
    let key_string =  String::from(resouce_str);
    key_string
  } 

  // fn gen_random_str(&mut self) -> String {
  //   let len = self.length_distrib.sample(&mut self.rng);
  //   let random_str = 
    
  //   random_str
  // } 

  pub fn get_next_op(&mut self) -> Operation {
    let categ  = self.generate_op();
    let key_str = self.gen_random_key();
    let mut body_str = String::from("");
    if categ == 0 {
      body_str = self.random_str.clone();
    }
    Operation { category: categ, key: key_str, body: body_str }
  }
}