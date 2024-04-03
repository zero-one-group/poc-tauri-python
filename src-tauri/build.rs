fn main() {
    // Read value from `.env` file (compile time)
    dotenv_build::output(dotenv_build::Config::default()).unwrap();
    tauri_build::build()
}
