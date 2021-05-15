use iced::{button, container, rule, text_input, Background, Color};

pub struct TimerStyle {
    pub is_running: bool,
    pub opacity: f32,
}

impl container::StyleSheet for TimerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(match self.is_running {
                true => Color::from_rgba8(0x00, 0x90, 0x40, self.opacity),
                false => Color::from_rgba8(0xc8, 0x40, 0x00, self.opacity),
            }),
            ..container::Style::default()
        }
    }
}

pub struct TextInputStyle;

impl text_input::StyleSheet for TextInputStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            border_color: Color::from_rgb8(0x60, 0x60, 0x60),
            border_radius: 0.0,
            border_width: 1.0,
            ..text_input::Style::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style { ..self.active() }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb8(0x90, 0x90, 0x90)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb8(0x00, 0x00, 0x00)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb8(0xb0, 0xb0, 0xb0)
    }
}

pub struct RuleStyle;

impl rule::StyleSheet for RuleStyle {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: Color::from_rgb8(0x90, 0x90, 0x90),
            width: 1,
            radius: 0.0,
            fill_mode: rule::FillMode::Padded(8),
        }
    }
}

pub struct TrackedTimeStyle;

impl container::StyleSheet for TrackedTimeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0xc8, 0xc8, 0xc8))),
            ..container::Style::default()
        }
    }
}

pub struct IndexStyle;

impl container::StyleSheet for IndexStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0xff, 0xb0, 0x60))),
            border_width: 0.0,
            ..container::Style::default()
        }
    }
}

pub struct TooltipStyle;

impl container::StyleSheet for TooltipStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(0xee, 0xee, 0xee)),
            background: Some(Background::Color(Color::from_rgb8(0x40, 0x40, 0x40))),
            ..container::Style::default()
        }
    }
}

pub struct ButtonStyle {
    pub foreground: Option<Color>,
}

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0xff, 0xff, 0xff))),
            border_color: Color::from_rgb8(0x60, 0x60, 0x60),
            border_radius: 2.0,
            border_width: 1.0,
            text_color: self
                .foreground
                .unwrap_or(Color::from_rgb8(0x00, 0x00, 0x00)),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style { ..self.active() }
    }

    fn pressed(&self) -> button::Style {
        button::Style { ..self.hovered() }
    }
}
