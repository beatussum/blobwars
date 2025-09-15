//! A module contaning implementation of the different [widgets](ratatui::widgets::Widget) used by the application

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Wrap},
};

use ratatui_macros::{line, span, text};

pub mod board;

/// A theme
///
/// Instances of this `struct` are used to colorize text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Theme {
    /// Style to emphasize
    pub emph: Style,

    /// Style to mark as important
    pub important: Style,

    /// Style used by links
    pub link: Style,

    /// Primary style
    pub primary: Style,

    /// Secondary style
    pub secondary: Style,

    /// Tertiary style
    pub tertiary: Style,

    /// Style used by titles
    pub title: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            emph: Style::default().bold().italic().green(),
            important: Style::default().bold().fg(Color::Rgb(0xe5, 0x95, 0x00)),
            link: Style::default().fg(Color::Rgb(0xf4, 0xb8, 0x60)),
            primary: Style::default(),
            secondary: Style::default().fg(Color::Rgb(0x7e, 0x89, 0x87)),
            tertiary: Style::default().fg(Color::Rgb(0x4b, 0x4a, 0x67)),
            title: Style::default().bold().italic().underlined(),
        }
    }
}

/// Widget showing application credits
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Credits {
    /// The [theme](Theme) used to colorize text
    pub theme: Theme,
}

impl Widget for Credits {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Credits");

        let text = text![
            line![
                "This program was written by ",
                span!(self.theme.important; "Mattéo Rossillol‑‑Laruelle"),
                " <",
                span!(self.theme.link; "beatussum@protonmail.com"),
                "> (a.k.a. ",
                span!(self.theme.important; "@beatussum"),
                ") and is licenced under ",
                span!(self.theme.important; "GPL-3.0-or-later"),
                ".",
            ],
            line![],
            line![
                "If you want to support my work, do not forget to ",
                span!(self.theme.important; "star"),
                " and ",
                span!(self.theme.important; "follow"),
                " the ",
                span!(self.theme.secondary; "GitHub repository"),
                " at ",
                span!(self.theme.link; "https://github.com/beatussum/blobwars"),
                ".",
            ],
            line![],
            span!(self.theme.title; "Licence notice:"),
            line![],
            line![
                span!(self.theme.secondary; "blobwars"),
                " Copyright (C) ",
                span!(self.theme.tertiary; "2025"),
                " ",
                span!(self.theme.important; "Mattéo Rossillol‑‑Laruelle"),
                " <",
                span!(self.theme.link; "beatussum@protonmail.com"),
                ">",
            ],
            line!["This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'."],
            line!["This is free software, and you are welcome to redistribute it"],
            line!["under certain conditions; type `show c' for details."],
        ];

        Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

/// A [`Widget`] representing the logo of the application
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Logo {
    /// The [theme](Theme) used to colorize text
    pub theme: Theme,
}

impl Logo {
    const B_TEXT: [&'static str; Self::HEIGHT] = [
        r##"     ...     ..     "##,
        r##"  .=*8888x <"?88h.  "##,
        r##" X>  '8888H> '8888  "##,
        r##"'88h. `8888   8888  "##,
        r##"'8888 '8888    "88> "##,
        r##" `888 '8888.xH888x. "##,
        r##"   X" :88*~  `*8888>"##,
        r##" ~"   !"`      "888>"##,
        r##"  .H8888h.      ?88 "##,
        r##" :"^"88888h.    '!  "##,
        r##" ^    "88888hx.+"   "##,
        r##"        ^"**""      "##,
    ];

    const LOB_TEXT: [&'static str; Self::HEIGHT] = [
        r##"      ..                   ..   "##,
        r##"x .d88"              . uW8"     "##,
        r##" 5888R          u.   `t888      "##,
        r##" '888R    ...ue888b   8888   .  "##,
        r##"  888R    888R Y888r  9888.z88N "##,
        r##"  888R    888R I888>  9888  888E"##,
        r##"  888R    888R I888>  9888  888E"##,
        r##"  888R    888R I888>  9888  888E"##,
        r##"  888R   u8888cJ888   9888  888E"##,
        r##" .888B .  "*888*P"   .8888  888""##,
        r##" ^*888%     'Y"       `%888*%"  "##,
        r##"   "%                    "`     "##,
    ];

    const W_TEXT: [&'static str; Self::HEIGHT] = [
        r##"     ...    .     ...     "##,
        r##"  .~`"888x.!**h.-``888h.  "##,
        r##" dX   `8888   :X   48888> "##,
        r##"'888x  8888  X88.  '8888> "##,
        r##"'88888 8888X:8888:   )?""`"##,
        r##" `8888>8888 '88888>.88h.  "##,
        r##"   `8" 888f  `8888>X88888."##,
        r##"  -~` '8%"     88" `88888X"##,
        r##"  .H888n.      XHn.  `*88!"##,
        r##" :88888888x..x88888X.  `! "##,
        r##" f  ^%888888% `*88888nx"  "##,
        r##"      `"**"`    `"**""    "##,
    ];

    const ARS_TEXT: [&'static str; Self::HEIGHT] = [
        r##"                            .x+=:.  "##,
        r##"                           z`    ^% "##,
        r##"               .u    .        .   <k"##,
        r##"      u      .d88B :@8c     .@8Ned8""##,
        r##"   us888u.  ="8888f8888r  .@^%8888" "##,
        r##".@88 "8888"   4888>'88"  x88:  `)8b."##,
        r##"9888  9888    4888> '    8888N=*8888"##,
        r##"9888  9888    4888>       %8"    R88"##,
        r##"9888  9888   .d888L .+     @8Wou 9% "##,
        r##"9888  9888   ^"8888*"    .888888P`  "##,
        r##""888*""888"     "Y"      `   ^"F    "##,
        r##" ^Y"   ^Y'                          "##,
    ];

    const HEIGHT: usize = 12;

    const B_WIDTH: u16 = Self::B_TEXT[0].len() as _;
    const LOB_WIDTH: u16 = Self::LOB_TEXT[0].len() as _;
    const W_WIDTH: u16 = Self::W_TEXT[0].len() as _;
    const ARS_WIDTH: u16 = Self::ARS_TEXT[0].len() as _;

    const MARGIN_WIDTH: u16 = Self::B_WIDTH;
    const OFFSET_WIDTH: u16 = 2;
}

impl Widget for Logo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top_area, bottom_area] =
            Layout::vertical([Self::HEIGHT as u16, Self::HEIGHT as u16]).areas(area);

        let [b_area, _, lob_area] =
            Layout::horizontal([Self::B_WIDTH, Self::OFFSET_WIDTH, Self::LOB_WIDTH])
                .areas(top_area);

        Text::from_iter(Self::B_TEXT)
            .style(self.theme.secondary)
            .render(b_area, buf);

        Text::from_iter(Self::LOB_TEXT)
            .style(self.theme.tertiary)
            .render(lob_area, buf);

        let [_, w_area, _, ars_area] = Layout::horizontal([
            Self::MARGIN_WIDTH,
            Self::W_WIDTH,
            Self::OFFSET_WIDTH,
            Self::ARS_WIDTH,
        ])
        .areas(bottom_area);

        Text::from_iter(Self::W_TEXT)
            .style(self.theme.secondary)
            .render(w_area, buf);

        Text::from_iter(Self::ARS_TEXT)
            .style(self.theme.tertiary)
            .render(ars_area, buf);
    }
}
