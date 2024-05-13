

fn handler(mut data: HashMap<String, Value>)->HashMap<String, Value>{
    data.insert("rust".to_string(), json!("Hello, world!"));
    return data;
}
