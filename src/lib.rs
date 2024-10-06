use wasm_bindgen::prelude::*;

// return Hello World, $name
#[wasm_bindgen]
pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello("World");
        assert_eq!(result, "Hello, World!");
    }
}
