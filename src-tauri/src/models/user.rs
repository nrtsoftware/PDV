#[tauri::command]
pub async fn login_user(name: &str, password: &str) -> Result<String, String> {
    let user = User {
        name: name,
        password: password,
    };
    let conn: Connection = Connection::open("../app.sqlite3").map_err(|err| format!("{}", err))?;
    let mut stmt = conn.prepare("SELECT id, name, password FROM person WHERE name = ?1")
        .map_err(|err| format!("{}", err))?;

    let mut rows = stmt.query([&user.name]).map_err(|err| format!("{}", err))?;
    if let Some(row) = rows.next().map_err(|err| format!("{}", err))? {
        let password: String = row.get(2).map_err(|err| format!("{}", err))?;
        if password == user.password {
            return Ok(format!("Funcionou!, {}! Voce está logado!", name));
        }
    }

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}
