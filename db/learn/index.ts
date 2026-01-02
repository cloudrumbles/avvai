import { Database } from 'bun:sqlite';

const db = new Database("app.db");

db.run(`
    CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    name TEXT
    )
`);

db.run('INSERT OR IGNORE INTO users VALUES (1, "shah")');
db.run('INSERT OR IGNORE INTO users VALUES (2, "yasmin")');

const users = db.query('SELECT * FROM users').all()
console.log(users)

const shah = db.query('SELECT * FROM users WHERE name = "shah"').all();
console.log(shah);
