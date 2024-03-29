#[macro_export]
macro_rules! vspd {
    ($($nm:expr => $v:expr,)*) => {{
        let samples = vec![
            $($crate::vspd::Sample::new($nm, $v),)*
        ];
        VSPD::new(samples)
    }};
    ($($nm:expr =>$v:expr),*) => {{
        let samples = vec![
            $($crate::vspd::Sample::new($nm, $v),)*
        ];
        VSPD::new(samples)
    }};
}
