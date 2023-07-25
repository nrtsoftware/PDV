trait ModelTrait {
    fn show() -> string;
}

struct Model {
    conn: &str,
}

impl Model for ModelTrait {
    fn show() {}
}