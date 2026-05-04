#[derive(Debug)]
pub struct Value(pub f32);

pub fn write_value_array(constants: &mut Vec<Value>, value: Value) {
    constants.push(value);
}

pub fn free_value_array(constants: &mut Vec<Value>) {
    constants.clear();
}

pub fn print_value(value: &Value) {
    println!("{:?}", value);
}
