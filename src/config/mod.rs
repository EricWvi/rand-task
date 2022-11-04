use tokio::sync::OnceCell;

static DB_URL: OnceCell<String> = OnceCell::const_new();
static TASK_DIR: OnceCell<String> = OnceCell::const_new();

pub fn init() {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("rand-task.env");
    while !path.exists() {
        path.pop();
        path.pop();
        path.push("rand-task.env");
    }
    dotenv::from_filename(path).ok();

    DB_URL
        .set(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .expect("DB_URL can not be set");

    TASK_DIR
        .set(std::env::var("TASK_DIR").expect("TASK_DIR must be set"))
        .expect("TASK_DIR can not be set");
}

#[inline]
pub fn db_url() -> &'static String {
    DB_URL.get().unwrap()
}

#[inline]
pub fn task_dir() -> &'static String {
    TASK_DIR.get().unwrap()
}
