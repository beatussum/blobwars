//! The implementation module of [`Logo`]

use ratatui::{prelude::*, widgets::Widget};

/// A [`Widget`] representing the logo of the application
pub struct Logo;

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

    const CAPITAL_STYLE: Style = Style::new().fg(Color::Rgb(0xce, 0x4b, 0x27));
    const BASIC_STYLE: Style = Style::new().fg(Color::Rgb(0x41, 0x5a, 0xb4));
}

impl Widget for Logo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top_area, bottom_area] =
            Layout::vertical([Self::HEIGHT as u16, Self::HEIGHT as u16]).areas(area);

        let [b_area, _, lob_area] =
            Layout::horizontal([Self::B_WIDTH, Self::OFFSET_WIDTH, Self::LOB_WIDTH])
                .areas(top_area);

        Text::from_iter(Self::B_TEXT)
            .style(Self::CAPITAL_STYLE)
            .render(b_area, buf);

        Text::from_iter(Self::LOB_TEXT)
            .style(Self::BASIC_STYLE)
            .render(lob_area, buf);

        let [_, w_area, _, ars_area] = Layout::horizontal([
            Self::MARGIN_WIDTH,
            Self::W_WIDTH,
            Self::OFFSET_WIDTH,
            Self::ARS_WIDTH,
        ])
        .areas(bottom_area);

        Text::from_iter(Self::W_TEXT)
            .style(Self::CAPITAL_STYLE)
            .render(w_area, buf);

        Text::from_iter(Self::ARS_TEXT)
            .style(Self::BASIC_STYLE)
            .render(ars_area, buf);
    }
}
