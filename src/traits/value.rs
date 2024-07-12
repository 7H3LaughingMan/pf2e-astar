use rapier2d::na::Point2;
use wasm_bindgen::JsValue;

pub trait Value
where Self: Sized
{
    fn get(&self, key: &str) -> JsValue;
    fn get_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> T;
    fn get_point(&self, key: &str) -> Point2<f32>;
}

impl Value for JsValue {
    fn get(&self, key: &str) -> JsValue {
        js_sys::Reflect::get(self, &JsValue::from_str(key)).unwrap()
    }

    fn get_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> T {
        serde_wasm_bindgen::from_value(self.get(key)).unwrap()
    }

    fn get_point(&self, key: &str) -> Point2<f32> {
        let point = self.get(key);
        let x = point.get_value("x");
        let y = point.get_value("y");

        Point2::<f32>::new(x, y)
    }
}
