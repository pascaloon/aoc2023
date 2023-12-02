#[macro_export]
macro_rules! make_days_map {
    ($fn_name: ident, $fn_content_name: ident, {$($es:expr => $mod:ident),*}) => (
        fn $fn_name(day: u8, part: u8) {
            match (day, part) {
                $(($es, 1u8) => $mod::part1($fn_content_name(day, part)), ($es, 2u8) => $mod::part2($fn_content_name(day, part)),)*
                (_, _) => panic!("Couldn't find day for ({}, {})", day, part)
            };
        }
    )
}