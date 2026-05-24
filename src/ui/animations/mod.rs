#[allow(dead_code)]
pub fn spinner_frames() -> [&'static str; 4] {
    ["[=   ]", "[==  ]", "[=== ]", "[====]"]
}

#[allow(dead_code)]
pub fn pulse_frames() -> [&'static str; 4] {
    ["#", "=", "-", "."]
}

pub fn slot_reel_frames() -> [&'static str; 12] {
    [
        "[>         ]",
        "[>>        ]",
        "[ >>>      ]",
        "[  >>>>    ]",
        "[   >>>>>  ]",
        "[    >>>>>>]",
        "[   <<<<<< ]",
        "[  <<<<<   ]",
        "[ <<<<     ]",
        "[<<<       ]",
        "[<<        ]",
        "[<         ]",
    ]
}
