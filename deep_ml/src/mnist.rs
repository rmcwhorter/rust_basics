extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
use std::fs;
use std::collections::HashMap;

extern crate ndarray;
use self::ndarray::{Array, Array2};

pub fn mnist() -> HashMap<String, Vec<Array2<f64>>> {
    let base_path = "src/digits/";
    let extension = ".json";
    let filenames = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut db: HashMap<String, Vec<Array2<f64>>> = HashMap::new();

    for file in filenames {
        db.insert(file.to_string(), vec![]);
        let current_path = format!("{}{}{}", base_path, file, extension);
        let mut string: String = fs::read_to_string(current_path).unwrap().parse().unwrap();
        let hash: HashMap<String, Vec<f64>> = serde_json::from_str(&string).unwrap();
        
        for a in 0..hash["data"].len()/28/28{
            let ray = Array::from_shape_vec((28,28), hash["data"][a*784..(a+1)*784].to_vec());
            db.get_mut(file).unwrap().push(ray.unwrap());
        }
    }
    return db;
}