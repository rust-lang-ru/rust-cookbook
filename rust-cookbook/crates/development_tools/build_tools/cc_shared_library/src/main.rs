use anyhow::Result;
use std::ffi::CString;
use std::os::raw::c_char;

unsafe extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
    fn greet(name: *const c_char);
}

fn prompt(s: &str) -> Result<String> {
    use std::io::Write;
    print!("{s}");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// Safe wrapper around the unsafe `add_numbers` C function.
fn library_add_numbers(a: i32, b: i32) -> i32 {
    unsafe { add_numbers(a, b) }
}

// Safe wrapper around the unsafe `greet` C function.
fn library_greet(name: &CString) {
    unsafe { greet(name.as_ptr()) }
}

fn main() -> Result<()> {
    let name = prompt("What's your name? ")?;
    let c_name = CString::new(name)?;
    library_greet(&c_name);
    let result = library_add_numbers(10, 12);
    println!("Addition result: {result}");
    Ok(())
}
