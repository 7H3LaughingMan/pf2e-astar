use wasm_bindgen::JsValue;

pub trait Value
where
    Self: Sized,
{
    fn get(&self, key: &str) -> JsValue;
    fn get_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> T;
}

impl Value for JsValue {
    fn get(&self, key: &str) -> JsValue {
        js_sys::Reflect::get(self, &JsValue::from_str(key)).unwrap()
    }

    fn get_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> T {
        serde_wasm_bindgen::from_value(self.get(key)).unwrap()
    }
}
