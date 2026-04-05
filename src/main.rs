// 1. Declare your "World" (The 3 Folders)
mod io;
mod logic;
mod storage;

fn main() {
    // 2. IO: The Senses grab data
    let input = io::get_user_input();

    // 3. MAIN: The Bouncer validates entry
    if input.is_empty() {
        io::print_error("Access Denied: Empty Input");
        return;
    }

    // 4. LOGIC: The Brain processes it
    let result = logic::calculate(input);

    // 5. STORAGE: The Vault saves it
    storage::save_data(result);
}
