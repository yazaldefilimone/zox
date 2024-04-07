use rusty_jsc::JSContext;

fn main() {
    let mut context = JSContext::default();
    match context.evaluate_script("'Hello World!'", 1) {
        Ok(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_string(&context).unwrap())
        }
    }
}
