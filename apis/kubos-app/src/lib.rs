#[macro_use]
extern crate serde_json;

pub fn app_var() -> u32 {
    json!(1).as_u64().unwrap() as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
