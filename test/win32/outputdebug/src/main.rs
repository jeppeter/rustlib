

mod wchar_windows;
mod loglib_windows;

fn main() {
    loglib_windows::win_output_debug("hello world\n");
}
