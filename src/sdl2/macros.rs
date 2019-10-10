#[macro_export]
macro_rules! try_sdl2 {
  ($name:ident, $($arg:expr),* $(,)*) => {
    #[allow(unused_unsafe)]
    unsafe {
      if $name($($arg,)*) != 0 {
        panic!("[x] {}: {}", stringify!($name), $crate::sdl2::error());
      }
    }
  };
}
