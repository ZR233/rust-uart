use kernel::bindings::sbi_console_put;


pub fn _print(s: &str) {
    unsafe {
        for c in s.as_bytes() {
            sbi_console_put(*c as _);
        }
        sbi_console_put('\n' as _);
    }
}

#[macro_export]
macro_rules! pr_println {
    ($($arg:tt)*) => {unsafe{
        let s = kernel::str::CString::try_from_fmt(kernel::fmt!($($arg)*)).unwrap();
        $crate::sbi_print::_print(s.as_str_unchecked());
    }};
}
