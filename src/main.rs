/* 
1. Configuration CRUD2. Configuration CRUD
2. Namespace entry state management
3. .env CRUD
4. encryption
*/

mod args;
mod db;

fn main() {
    args::args::Args::tst();
    db::keys::keyCrud::init(String::from("Test"));
}
