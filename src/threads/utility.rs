use piston::input::keyboard::Key;
pub fn key_from_u64(n: u64) -> Option<Key> {
    match n {
        0 => Some(Key::Unknown),
        8 => Some(Key::Backspace),
        9 => Some(Key::Tab),
        13 => Some(Key::Return),
        27 => Some(Key::Escape),
        32 => Some(Key::Space),
        33 => Some(Key::Exclaim),
        34 => Some(Key::Quotedbl),
        35 => Some(Key::Hash),
        36 => Some(Key::Dollar),
        37 => Some(Key::Percent),
        38 => Some(Key::Ampersand),
        39 => Some(Key::Quote),
        40 => Some(Key::LeftParen),
        41 => Some(Key::RightParen),
        42 => Some(Key::Asterisk),
        43 => Some(Key::Plus),
        44 => Some(Key::Comma),
        45 => Some(Key::Minus),
        46 => Some(Key::Period),
        47 => Some(Key::Slash),
        48 => Some(Key::D0),
        49 => Some(Key::D1),
        50 => Some(Key::D2),
        51 => Some(Key::D3),
        52 => Some(Key::D4),
        53 => Some(Key::D5),
        54 => Some(Key::D6),
        55 => Some(Key::D7),
        56 => Some(Key::D8),
        57 => Some(Key::D9),
        58 => Some(Key::Colon),
        59 => Some(Key::Semicolon),
        60 => Some(Key::Less),
        61 => Some(Key::Equals),
        62 => Some(Key::Greater),
        63 => Some(Key::Question),
        64 => Some(Key::At),
        91 => Some(Key::LeftBracket),
        92 => Some(Key::Backslash),
        93 => Some(Key::RightBracket),
        94 => Some(Key::Caret),
        95 => Some(Key::Underscore),
        96 => Some(Key::Backquote),
        97 => Some(Key::A),
        98 => Some(Key::B),
        99 => Some(Key::C),
        100 => Some(Key::D),
        101 => Some(Key::E),
        102 => Some(Key::F),
        103 => Some(Key::G),
        104 => Some(Key::H),
        105 => Some(Key::I),
        106 => Some(Key::J),
        107 => Some(Key::K),
        108 => Some(Key::L),
        109 => Some(Key::M),
        110 => Some(Key::N),
        111 => Some(Key::O),
        112 => Some(Key::P),
        113 => Some(Key::Q),
        114 => Some(Key::R),
        115 => Some(Key::S),
        116 => Some(Key::T),
        117 => Some(Key::U),
        118 => Some(Key::V),
        119 => Some(Key::W),
        120 => Some(Key::X),
        121 => Some(Key::Y),
        122 => Some(Key::Z),
        127 => Some(Key::Delete),
        1073741881 => Some(Key::CapsLock),
        1073741882 => Some(Key::F1),
        1073741883 => Some(Key::F2),
        1073741884 => Some(Key::F3),
        1073741885 => Some(Key::F4),
        1073741886 => Some(Key::F5),
        1073741887 => Some(Key::F6),
        1073741888 => Some(Key::F7),
        1073741889 => Some(Key::F8),
        1073741890 => Some(Key::F9),
        1073741891 => Some(Key::F10),
        1073741892 => Some(Key::F11),
        1073741893 => Some(Key::F12),
        1073741894 => Some(Key::PrintScreen),
        1073741895 => Some(Key::ScrollLock),
        1073741896 => Some(Key::Pause),
        1073741897 => Some(Key::Insert),
        1073741898 => Some(Key::Home),
        1073741899 => Some(Key::PageUp),
        1073741901 => Some(Key::End),
        1073741902 => Some(Key::PageDown),
        1073741903 => Some(Key::Right),
        1073741904 => Some(Key::Left),
        1073741905 => Some(Key::Down),
        1073741906 => Some(Key::Up),
        1073741907 => Some(Key::NumLockClear),
        1073741908 => Some(Key::NumPadDivide),
        1073741909 => Some(Key::NumPadMultiply),
        1073741910 => Some(Key::NumPadMinus),
        1073741911 => Some(Key::NumPadPlus),
        1073741912 => Some(Key::NumPadEnter),
        1073741913 => Some(Key::NumPad1),
        1073741914 => Some(Key::NumPad2),
        1073741915 => Some(Key::NumPad3),
        1073741916 => Some(Key::NumPad4),
        1073741917 => Some(Key::NumPad5),
        1073741918 => Some(Key::NumPad6),
        1073741919 => Some(Key::NumPad7),
        1073741920 => Some(Key::NumPad8),
        1073741921 => Some(Key::NumPad9),
        1073741922 => Some(Key::NumPad0),
        1073741923 => Some(Key::NumPadPeriod),
        1073741925 => Some(Key::Application),
        1073741926 => Some(Key::Power),
        1073741927 => Some(Key::NumPadEquals),
        1073741928 => Some(Key::F13),
        1073741929 => Some(Key::F14),
        1073741930 => Some(Key::F15),
        1073741931 => Some(Key::F16),
        1073741932 => Some(Key::F17),
        1073741933 => Some(Key::F18),
        1073741934 => Some(Key::F19),
        1073741935 => Some(Key::F20),
        1073741936 => Some(Key::F21),
        1073741937 => Some(Key::F22),
        1073741938 => Some(Key::F23),
        1073741939 => Some(Key::F24),
        1073741940 => Some(Key::Execute),
        1073741941 => Some(Key::Help),
        1073741942 => Some(Key::Menu),
        1073741943 => Some(Key::Select),
        1073741944 => Some(Key::Stop),
        1073741945 => Some(Key::Again),
        1073741946 => Some(Key::Undo),
        1073741947 => Some(Key::Cut),
        1073741948 => Some(Key::Copy),
        1073741949 => Some(Key::Paste),
        1073741950 => Some(Key::Find),
        1073741951 => Some(Key::Mute),
        1073741952 => Some(Key::VolumeUp),
        1073741953 => Some(Key::VolumeDown),
        1073741957 => Some(Key::NumPadComma),
        1073741958 => Some(Key::NumPadEqualsAS400),
        1073741977 => Some(Key::AltErase),
        1073741978 => Some(Key::Sysreq),
        1073741979 => Some(Key::Cancel),
        1073741980 => Some(Key::Clear),
        1073741981 => Some(Key::Prior),
        1073741982 => Some(Key::Return2),
        1073741983 => Some(Key::Separator),
        1073741984 => Some(Key::Out),
        1073741985 => Some(Key::Oper),
        1073741986 => Some(Key::ClearAgain),
        1073741987 => Some(Key::CrSel),
        1073741988 => Some(Key::ExSel),
        1073742000 => Some(Key::NumPad00),
        1073742001 => Some(Key::NumPad000),
        1073742002 => Some(Key::ThousandsSeparator),
        1073742003 => Some(Key::DecimalSeparator),
        1073742004 => Some(Key::CurrencyUnit),
        1073742005 => Some(Key::CurrencySubUnit),
        1073742006 => Some(Key::NumPadLeftParen),
        1073742007 => Some(Key::NumPadRightParen),
        1073742008 => Some(Key::NumPadLeftBrace),
        1073742009 => Some(Key::NumPadRightBrace),
        1073742010 => Some(Key::NumPadTab),
        1073742011 => Some(Key::NumPadBackspace),
        1073742012 => Some(Key::NumPadA),
        1073742013 => Some(Key::NumPadB),
        1073742014 => Some(Key::NumPadC),
        1073742015 => Some(Key::NumPadD),
        1073742016 => Some(Key::NumPadE),
        1073742017 => Some(Key::NumPadF),
        1073742018 => Some(Key::NumPadXor),
        1073742019 => Some(Key::NumPadPower),
        1073742020 => Some(Key::NumPadPercent),
        1073742021 => Some(Key::NumPadLess),
        1073742022 => Some(Key::NumPadGreater),
        1073742023 => Some(Key::NumPadAmpersand),
        1073742024 => Some(Key::NumPadDblAmpersand),
        1073742025 => Some(Key::NumPadVerticalBar),
        1073742026 => Some(Key::NumPadDblVerticalBar),
        1073742027 => Some(Key::NumPadColon),
        1073742028 => Some(Key::NumPadHash),
        1073742029 => Some(Key::NumPadSpace),
        1073742030 => Some(Key::NumPadAt),
        1073742031 => Some(Key::NumPadExclam),
        1073742032 => Some(Key::NumPadMemStore),
        1073742033 => Some(Key::NumPadMemRecall),
        1073742034 => Some(Key::NumPadMemClear),
        1073742035 => Some(Key::NumPadMemAdd),
        1073742036 => Some(Key::NumPadMemSubtract),
        1073742037 => Some(Key::NumPadMemMultiply),
        1073742038 => Some(Key::NumPadMemDivide),
        1073742039 => Some(Key::NumPadPlusMinus),
        1073742040 => Some(Key::NumPadClear),
        1073742041 => Some(Key::NumPadClearEntry),
        1073742042 => Some(Key::NumPadBinary),
        1073742043 => Some(Key::NumPadOctal),
        1073742044 => Some(Key::NumPadDecimal),
        1073742045 => Some(Key::NumPadHexadecimal),
        1073742048 => Some(Key::LCtrl),
        1073742049 => Some(Key::LShift),
        1073742050 => Some(Key::LAlt),
        1073742051 => Some(Key::LGui),
        1073742052 => Some(Key::RCtrl),
        1073742053 => Some(Key::RShift),
        1073742054 => Some(Key::RAlt),
        1073742055 => Some(Key::RGui),
        1073742081 => Some(Key::Mode),
        1073742082 => Some(Key::AudioNext),
        1073742083 => Some(Key::AudioPrev),
        1073742084 => Some(Key::AudioStop),
        1073742085 => Some(Key::AudioPlay),
        1073742086 => Some(Key::AudioMute),
        1073742087 => Some(Key::MediaSelect),
        1073742088 => Some(Key::Www),
        1073742089 => Some(Key::Mail),
        1073742090 => Some(Key::Calculator),
        1073742091 => Some(Key::Computer),
        1073742092 => Some(Key::AcSearch),
        1073742093 => Some(Key::AcHome),
        1073742094 => Some(Key::AcBack),
        1073742095 => Some(Key::AcForward),
        1073742096 => Some(Key::AcStop),
        1073742097 => Some(Key::AcRefresh),
        1073742098 => Some(Key::AcBookmarks),
        1073742099 => Some(Key::BrightnessDown),
        1073742100 => Some(Key::BrightnessUp),
        1073742101 => Some(Key::DisplaySwitch),
        1073742102 => Some(Key::KbdIllumToggle),
        1073742103 => Some(Key::KbdIllumDown),
        1073742104 => Some(Key::KbdIllumUp),
        1073742105 => Some(Key::Eject),
        1073742106 => Some(Key::Sleep),

        _ => Some(Key::Unknown)
    }
}