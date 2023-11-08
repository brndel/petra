
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature="ssr")] {
        use std::path::Path;
        use once_cell::sync::OnceCell;
        use std::sync::Mutex;
        use std::sync::MutexGuard;
        use mensula::Database;

        use crate::api;

        static DATABASE: OnceCell<Mutex<Database>> = OnceCell::new();

        pub fn init<P: AsRef<Path>>(path: P) {
            let mut db = Database::open(path).expect("could not open db");

            api::register_tables(&mut db).unwrap();
            
            DATABASE.set(Mutex::new(db)).expect("db already initialized");
        }

        pub fn get_db() -> MutexGuard<'static, Database> {
            DATABASE.get().expect("database not initialized yet").lock().unwrap()
        }
    }
}