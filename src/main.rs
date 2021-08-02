mod hook;

fn main() {
    hook::run_hook();

    std::thread::park(); // Let the keyboard hook keep running
}
