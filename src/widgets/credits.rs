//! Implementation of [`Credits`]

use ratatui::{prelude::*, widgets::Block};
use ratatui_macros::{line, span, text};

/// Widget showing application credits
pub struct Credits;

impl Widget for Credits {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Credits");
        let inner = block.inner(area);

        let bold = Style::default().bold();
        let blue = bold.blue();
        let green = Style::default().green();
        let yellow = bold.yellow();

        let text = text![
            line![
                "This program was written by ",
                span!(yellow; "Mattéo Rossillol‑‑Laruelle"),
                " <",
                span!(green; "beatussum@protonmail.com"),
                ">",
            ],
            line![
                "(a.k.a. ",
                span!(yellow; "@beatussum"),
                ") and is licenced under ",
                span!(yellow; "GPL-3.0-or-later"),
                ".",
            ],
            line![],
            line![
                "If you want to support my work, do not forget to ",
                span!(yellow; "star"),
                " and ",
                span!(yellow; "follow"),
                " the",
            ],
            line![
                span!(blue; "GitHub repository"),
                " at ",
                span!(green; "https://github.com/beatussum/blobwars"),
                ".",
            ],
            line![],
            span!(bold.underlined(); "Licence notice:"),
            line![],
            line![
                span!(blue; "blobwars"),
                span!(bold; " Copyright (C) "),
                span!(blue; "2025"),
                " ",
                span!(yellow; "Mattéo Rossillol‑‑Laruelle"),
                span!(bold; " <"),
                span!(green; "beatussum@protonmail.com"),
                span!(bold; ">"),
            ],
            line!["This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'."],
            line!["This is free software, and you are welcome to redistribute it"],
            line!["under certain conditions; type `show c' for details."],
        ];

        Text::from_iter(text).render(inner, buf);
        block.render(area, buf);
    }
}
