use iced::{button, container, rule, text_input, Color};

const PRIMARY: Color = Color {
    r: 1.00,
    g: 0.6,
    b: 0.2,
    a: 1.0,
};

const DARK: Color = Color {
    r: 0.07,
    g: 0.07,
    b: 0.07,
    a: 1.0,
};

const DARK2: Color = Color {
    r: 0.14,
    g: 0.14,
    b: 0.14,
    a: 1.0,
};

const DARK3: Color = Color {
    r: 0.21,
    g: 0.21,
    b: 0.21,
    a: 1.0,
};

const DARK4: Color = Color {
    r: 0.28,
    g: 0.28,
    b: 0.28,
    a: 1.0,
};

const LIGHT_TEXT: Color = Color {
    r: 0.80,
    g: 0.80,
    b: 0.80,
    a: 1.0,
};

const LIGHT: Color = Color {
    r: 0.98,
    g: 0.98,
    b: 0.98,
    a: 1.0,
};

const LIGHT2: Color = Color {
    r: 0.93,
    g: 0.93,
    b: 0.93,
    a: 1.0,
};

const LIGHT3: Color = Color {
    r: 0.86,
    g: 0.86,
    b: 0.86,
    a: 1.0,
};

const LIGHT4: Color = Color {
    r: 0.79,
    g: 0.79,
    b: 0.79,
    a: 1.0,
};

const DARK_TEXT: Color = Color {
    r: 0.07,
    g: 0.07,
    b: 0.07,
    a: 1.0,
};

const GRAY: Color = Color {
    r: 0.6,
    g: 0.6,
    b: 0.6,
    a: 1.0,
};

pub struct RootStyle {
    pub is_dark_mode: bool,
}

impl container::StyleSheet for RootStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: if self.is_dark_mode {
                DARK.into()
            } else {
                LIGHT.into()
            },
            ..container::Style::default()
        }
    }
}

pub struct TextStyle {
    pub is_dark_mode: bool,
}

impl container::StyleSheet for TextStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: if self.is_dark_mode {
                LIGHT_TEXT.into()
            } else {
                DARK_TEXT.into()
            },
            ..container::Style::default()
        }
    }
}

pub struct TimerStyle {
    pub is_dark_mode: bool,
    pub is_running: bool,
    pub opacity: f32,
}

impl container::StyleSheet for TimerStyle {
    fn style(&self) -> container::Style {
        let o = self.opacity * if self.is_dark_mode { 0.5 } else { 1.0 };
        container::Style {
            text_color: Some(match self.is_running {
                true => Color::from_rgba8(0x00, 0x90, 0x40, o),
                false => Color::from_rgba8(0xc8, 0x40, 0x00, o),
            }),
            ..container::Style::default()
        }
    }
}

pub struct TextInputStyle {
    pub is_dark_mode: bool,
}

impl text_input::StyleSheet for TextInputStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: if self.is_dark_mode {
                DARK2.into()
            } else {
                LIGHT.into()
            },
            border_color: DARK4,
            border_radius: 0.0,
            border_width: if self.is_dark_mode { 0.0 } else { 1.0 },
            ..text_input::Style::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: if self.is_dark_mode {
                DARK2.into()
            } else {
                LIGHT2.into()
            },
            border_color: if self.is_dark_mode { GRAY } else { DARK4 },
            border_width: 1.0,
            ..self.active()
        }
    }

    fn placeholder_color(&self) -> Color {
        GRAY
    }

    fn value_color(&self) -> Color {
        if self.is_dark_mode {
            LIGHT_TEXT
        } else {
            DARK_TEXT
        }
    }

    fn selection_color(&self) -> Color {
        if self.is_dark_mode {
            GRAY
        } else {
            LIGHT4
        }
    }
}

pub struct RuleStyle;

impl rule::StyleSheet for RuleStyle {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: GRAY,
            width: 1,
            radius: 0.0,
            fill_mode: rule::FillMode::Padded(8),
        }
    }
}

pub struct TrackedTimeStyle {
    pub is_dark_mode: bool,
}

impl container::StyleSheet for TrackedTimeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: if self.is_dark_mode {
                Color::from_rgb(0.1, 0.1, 0.1).into()
            } else {
                LIGHT2.into()
            },
            ..container::Style::default()
        }
    }
}

pub struct IndexStyle {
    pub is_dark_mode: bool,
}

impl container::StyleSheet for IndexStyle {
    fn style(&self) -> container::Style {
        let mut c = PRIMARY;
        if self.is_dark_mode {
            c.a = 0.60;
        }
        container::Style {
            background: c.into(),
            border_width: 0.0,
            ..container::Style::default()
        }
    }
}

pub struct TooltipStyle;

impl container::StyleSheet for TooltipStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: LIGHT_TEXT.into(),
            background: DARK3.into(),
            ..container::Style::default()
        }
    }
}

pub struct ButtonStyle {
    pub is_dark_mode: bool,
    pub foreground: Option<Color>,
}

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: if self.is_dark_mode {
                DARK2.into()
            } else {
                LIGHT.into()
            },
            border_color: DARK3,
            border_radius: 2.0,
            border_width: if self.is_dark_mode { 0.0 } else { 1.0 },
            text_color: self.foreground.unwrap_or(if self.is_dark_mode {
                LIGHT_TEXT
            } else {
                DARK_TEXT
            }),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: if self.is_dark_mode {
                DARK3.into()
            } else {
                LIGHT2.into()
            },
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: if self.is_dark_mode {
                DARK4.into()
            } else {
                LIGHT3.into()
            },
            ..self.hovered()
        }
    }
}
