#[macro_export]
macro_rules! keys {
    ($($item:tt),* $(,)?) => {
        vec![ $( $crate::parse_key_combo!($item) ),* ]
    };
}

#[macro_export]
macro_rules! parse_key_combo {
    // Mouse N
    (Mouse $idx:literal) => {
        $crate::input::Key::from_mouse_index($idx)
    };

    // Комбинация с + - разбираем рекурсивно
    ($first:tt + $($rest:tt)+) => {{
        let mut keys = Vec::new();
        $crate::collect_key_codes!(keys, $first);
        $(
            $crate::collect_key_codes!(keys, $rest);
        )+
        $crate::input::Key::from_kb(keys)
    }};

    // Одиночная клавиша
    ($single:ident) => {
        $crate::input::prelude::Key::from_kb(vec![bevy::prelude::KeyCode::$single])
    };
}

#[macro_export]
macro_rules! collect_key_codes {
    ($vec:ident, $key:ident) => {
        $vec.push(bevy::prelude::KeyCode::$key);
    };
}
