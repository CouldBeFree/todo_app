use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        println!("\n\n{} is being created\n\n", title);
    }
}