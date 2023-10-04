use std::collections::HashMap;



pub enum Metric {

}

pub struct MetricId {
    name: String,
    group: String,
    description: String,
    //tags: Vec<String>,
    tags: HashMap<String, String>,
}