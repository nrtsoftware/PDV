use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

pub fn main () {
   // 1️⃣ Define migrations
let migrations = Migrations::new(vec![
    M::up("CREATE TABLE admin(email TEXT NOT NULL, password TEXT NOT NULL);"),
    // In the future, add more migrations here:
    //M::up("ALTER TABLE friend ADD COLUMN email TEXT;"),
]);

let mut conn = Connection::open("./app.sqlite3").unwrap();

// Apply some PRAGMA, often better to do it outside of migrations
conn.pragma_update(None, "journal_mode", &"WAL").unwrap();

// 2️⃣ Update the database schema, atomically
migrations.to_latest(&mut conn).unwrap();

// 3️⃣ Use the database 🥳
conn.execute("INSERT INTO admin (email, password) VALUES (?1, ?2)", params!["Nivs", "password"])
    .unwrap();
}
