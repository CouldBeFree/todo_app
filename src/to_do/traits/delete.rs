use serde_json::Map;
use serde_json::value::Value;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        println!("\n\n{} is being deleted\n\n", title);
    }
}