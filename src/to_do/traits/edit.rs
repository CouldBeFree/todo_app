use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use super::super::enums::TaskStatus;

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::DONE.stringify()));
        println!("\n\n{} is being set to done\n\n", title);
    }
    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::PENDING.stringify()));
        println!("\n\n{} is being set to pedning\n\n", title);
    }
}