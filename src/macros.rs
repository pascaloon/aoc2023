#[macro_export]
macro_rules! make_days_map {
    ($fn_name: ident, {$($es:expr => $mod:ident),*}) => (
        fn $fn_name(day: u8, part: u8, data: String) {
            match (day, part) {
                $(($es, 1u8) => {$mod::part1(data);}, ($es, 2u8) => {$mod::part2(data);},)*
                (_, _) => panic!("Couldn't find day for ({}, {})", day, part)
            };
        }
    )
}