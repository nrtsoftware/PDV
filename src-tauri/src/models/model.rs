use rusqlite::Connection;

pub struct Model {
    db: String,
}
impl Model {
    pub fn new(db: String) -> Model {
        Model {db}
    }

    pub fn connect(&self) -> std::result::Result<Connection, rusqlite::Error> {
        return Connection::open(&self.db.clone());
    }
}

pub fn model_connect() -> Model {
    let model: Model = Model::new("../sqlite3".to_string());
    return model.connect();
}