unsafe extern "C" {
    fn print_app_info();
}

fn main() {
    unsafe {
        print_app_info();
    }
}
