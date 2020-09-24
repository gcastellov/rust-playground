#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::default()
    };
    ( $($key:expr => $value:expr),+ $(,)? ) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $( hm.insert($key, $value); )*
            hm
        }
    };
}