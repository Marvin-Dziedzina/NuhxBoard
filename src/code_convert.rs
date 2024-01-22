pub fn code_convert(xinput_code: u32) -> u32 {
    // mouse buttons
    match xinput_code {
        // left
        1 => return 0,
        // right
        3 => return 1,
        _ => {}
    };
    // 1-9
    if (10..=18).contains(&xinput_code) {
        return xinput_code + 39;
    }
    // bckspce + tab
    if (22..=23).contains(&xinput_code) {
        return xinput_code - 14;
    }

    match xinput_code {
        // esc
        9 => 27,
        // 0
        19 => 48,
        // -
        20 => 189,
        // =
        21 => 187,
        // q
        24 => 81,
        // w
        25 => 87,
        // e
        26 => 69,
        // r
        27 => 82,
        // t
        28 => 84,
        // y
        29 => 89,
        // u
        30 => 85,
        // i
        31 => 73,
        // o
        32 => 79,
        // p
        33 => 80,
        // [
        34 => 219,
        // ]
        35 => 221,
        // enter
        36 => 13,
        // ctrl
        37 => 162,
        // a
        38 => 65,
        // s
        39 => 83,
        // d
        40 => 68,
        // f
        41 => 70,
        // g
        42 => 71,
        // h
        43 => 72,
        // j
        44 => 74,
        // k
        45 => 75,
        // l
        46 => 76,
        // ;
        47 => 186,
        // '
        48 => 222,
        // `
        49 => 192,
        // shift
        50 => 160,
        // \
        51 => 220,
        // z
        52 => 90,
        // x
        53 => 88,
        // c
        54 => 67,
        // v
        55 => 86,
        // b
        56 => 66,
        // n
        57 => 78,
        // m
        58 => 77,
        // ,
        59 => 188,
        // .
        60 => 190,
        // /
        61 => 191,
        // shift
        62 => 161,
        // *
        17 => 106,
        // alt
        64 => 18,
        // space
        65 => 32,
        // caps
        66 => 20,
        // F1
        67 => 112,
        // F2
        68 => 113,
        // F3
        69 => 114,
        // F4
        70 => 115,
        // F5
        71 => 116,
        // F6
        72 => 117,
        // F7
        73 => 118,
        // F8
        74 => 119,
        // F9
        75 => 120,
        // F10
        76 => 121,
        // F11
        95 => 122,
        // F12
        96 => 123,
        // NumLock
        77 => 144,
        // KP /
        106 => 111,
        // KP *
        63 => 106,
        // KP -
        82 => 109,
        // KP 7
        79 => 103,
        // KP 8
        80 => 104,
        // KP 9
        81 => 105,
        // KP +
        86 => 107,
        // KP 4
        83 => 100,
        // KP 5
        84 => 101,
        // KP 6
        85 => 102,
        // KP 1
        87 => 97,
        // KP 2
        88 => 98,
        // KP 3
        89 => 99,
        // KP 0
        90 => 96,
        // KP .
        91 => 110,
        // KP enter
        104 => 1025,
        // print screen
        107 => 44,
        // Pause
        127 => 19,
        // Insert
        118 => 45,
        // Del
        119 => 46,
        // Home
        110 => 36,
        // End
        115 => 35,
        // PgUp
        112 => 33,
        // PgDn
        117 => 34,
        // Up
        111 => 38,
        // Down
        116 => 40,
        // Left
        113 => 37,
        // Right
        114 => 39,
        _ => panic!("Unknown xinput code: {}", xinput_code),
    }
}
