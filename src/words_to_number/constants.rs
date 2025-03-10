pub(super) const NEGATIVE_PREFIX: &str = "منفی";

pub(super) static UNITS: &[(&str, i64)] = &[
    ("صفر", 0),
    ("یک", 1),
    ("دو", 2),
    ("سه", 3),
    ("چهار", 4),
    ("پنج", 5),
    ("شش", 6),
    ("شیش", 6),
    ("هفت", 7),
    ("هشت", 8),
    ("نه", 9),
    ("ده", 10),
    ("یازده", 11),
    ("دوازده", 12),
    ("سیزده", 13),
    ("چهارده", 14),
    ("پانزده", 15),
    ("شانزده", 16),
    ("هفده", 17),
    ("هجده", 18),
    ("نوزده", 19),
    ("بیست", 20),
    ("سی", 30),
    ("چهل", 40),
    ("پنجاه", 50),
    ("شصت", 60),
    ("هفتاد", 70),
    ("هشتاد", 80),
    ("نود", 90),
    ("صد", 100),
    ("یکصد", 100),
    ("دویست", 200),
    ("سیصد", 300),
    ("چهارصد", 400),
    ("پانصد", 500),
    ("ششصد", 600),
    ("هفتصد", 700),
    ("هشتصد", 800),
    ("نهصد", 900),
];

pub(super) static MAGNITUDE: &[(&str, i64)] = &[
    ("هزار", 1000),
    ("میلیون", 1000000),
    ("بیلیون", 1000000000),
    ("میلیارد", 1000000000),
    ("تریلیون", 1000000000000),
];

// pub(super) static ALL_WORDS: &[(&str, i64)] = UNITS.iter().chain(MAGNITUDE.iter());

pub(super) fn get_unit_number(unit: &str) -> Option<&i64> {
    let result = UNITS
        .iter()
        .find(|(key, _)| key == &unit)
        .map(|(_, details)| details);

    result
}

pub(super) fn get_magnitute_number(unit: &str) -> Option<&i64> {
    let result = MAGNITUDE
        .iter()
        .find(|(key, _)| key == &unit)
        .map(|(_, details)| details);

    result
}
