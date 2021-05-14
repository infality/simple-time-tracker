#![windows_subsystem = "windows"]
use iced::{
    button, executor, scrollable, text_input, time, tooltip, Application, Button, Clipboard, Color,
    Column, Command, Container, Element, Length, Row, Scrollable, Settings, Space, Subscription,
    Text, TextInput, Tooltip,
};

pub fn main() -> iced::Result {
    SimpleTimeTracker::run(Settings::default())
}

struct SimpleTimeTracker {
    is_running: bool,
    start_time: chrono::DateTime<chrono::Local>,
    pause_time: chrono::DateTime<chrono::Local>,
    tracked_times: Vec<TrackedTime>,

    start_stop_button: button::State,
    time_text_input: text_input::State,
    time_input: String,
    name_text_input: text_input::State,
    name_input: String,
    index_text_input: text_input::State,
    index_input: String,
    apply_operation_button: button::State,
    tracked_times_scroll: scrollable::State,
}

#[derive(Debug, Clone)]
struct TrackedTime {
    name: String,
    duration: chrono::Duration,

    copy_button: button::State,
    delete_button: button::State,
}

impl TrackedTime {
    fn new(name: String, duration: chrono::Duration) -> Self {
        TrackedTime {
            name,
            duration,
            copy_button: button::State::new(),
            delete_button: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TimeUpdate,
    StartStopTimer,
    TimeInputChanged(String),
    NameInputChanged(String),
    IndexInputChanged(String),
    ApplyOperation,
    DeleteTrackedTime(usize),
    CopyText(usize),
}

impl SimpleTimeTracker {
    fn apply_operation(&mut self) {
        let timer = match self.is_running {
            true => chrono::Local::now() - self.start_time,
            false => self.pause_time - self.start_time,
        };
        let mut duration = chrono::Duration::zero();

        // Parse time input
        if self.time_input.len() > 0 {
            let parts = self.time_input.split(':').collect::<Vec<&str>>();
            if parts.len() > 2 {
                return;
            }

            // Parse minutes
            let minutes = parts[0].parse();
            if let Ok(m) = minutes {
                if m >= 60 || (timer.num_hours() < 1 && m > timer.num_minutes()) {
                    return;
                }
                duration = duration.checked_add(&chrono::Duration::minutes(m)).unwrap();
            } else {
                return;
            }

            // Parse hours
            if parts.len() == 2 {
                let hours = parts[1].parse();
                if let Ok(h) = hours {
                    if h > timer.num_hours() {
                        return;
                    }
                    duration = duration.checked_add(&chrono::Duration::hours(h)).unwrap();
                } else {
                    return;
                }
            }
        } else {
            duration = timer;
        }

        // Ensure only either name or index is set
        if (self.name_input.len() > 0) == (self.index_input.len() > 0) {
            return;
        }

        if self.name_input.len() > 0 {
            self.tracked_times
                .push(TrackedTime::new(self.name_input.clone(), duration));
        } else {
            let index = self.index_input.parse::<usize>().unwrap();
            if index == 0 || index > self.tracked_times.len() {
                return;
            }

            self.tracked_times[index - 1].duration = match self.tracked_times[index - 1]
                .duration
                .checked_add(&duration)
            {
                Some(d) => d,
                None => return,
            };
        }
        self.time_input.clear();
        self.name_input.clear();
        self.index_input.clear();
        if self.is_running {
            self.start_time = chrono::Local::now();
        } else {
            self.start_time = chrono::Local::now();
            self.pause_time = self.start_time.clone();
        }
    }
}

impl Application for SimpleTimeTracker {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut temp = Vec::new();
        temp.push(TrackedTime {
            name: String::from("Test"),
            duration: chrono::Duration::hours(2),

            copy_button: button::State::new(),
            delete_button: button::State::new(),
        });
        temp.push(TrackedTime {
            name: String::from("Test"),
            duration: chrono::Duration::hours(2),

            copy_button: button::State::new(),
            delete_button: button::State::new(),
        });
        (
            Self {
                is_running: false,
                start_time: chrono::Local::now(),
                pause_time: chrono::Local::now(),
                tracked_times: temp,

                start_stop_button: button::State::new(),
                time_text_input: text_input::State::new(),
                time_input: String::new(),
                name_text_input: text_input::State::new(),
                name_input: String::new(),
                index_text_input: text_input::State::new(),
                index_input: String::new(),
                apply_operation_button: button::State::new(),
                tracked_times_scroll: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Simple Time Tracker")
    }

    fn update(&mut self, message: Message, clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::TimeUpdate => {}
            Message::StartStopTimer => {
                if self.is_running {
                    self.pause_time = chrono::Local::now();
                } else {
                    self.start_time = self.start_time + (chrono::Local::now() - self.pause_time);
                }
                self.is_running = !self.is_running;
            }
            Message::TimeInputChanged(input) => {
                if input.len() < 3 {
                    if input.chars().all(|c| c == ':' || c.is_numeric()) {
                        self.time_input = input;
                    }
                } else if input.len() < 6 {
                    if input.chars().all(|c| c == ':' || c.is_numeric())
                        && input.chars().any(|c| c == ':')
                    {
                        self.time_input = input;
                    }
                }
            }
            Message::NameInputChanged(input) => self.name_input = input,
            Message::IndexInputChanged(input) => {
                if input.len() == 0 || (input.len() < 3 && input.parse::<usize>().is_ok()) {
                    self.index_input = input
                }
            }
            Message::ApplyOperation => {
                self.apply_operation();
            }
            Message::DeleteTrackedTime(i) => drop(self.tracked_times.remove(i)),
            Message::CopyText(i) => clipboard.write(self.tracked_times[i].name.clone()),
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.is_running {
            true => time::every(std::time::Duration::from_millis(500)).map(|_| Message::TimeUpdate),
            false => Subscription::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let duration = match self.is_running {
            true => chrono::Local::now() - self.start_time,
            false => self.pause_time - self.start_time,
        };
        let time = Row::new()
            .push(
                Container::new(
                    Text::new(format!(
                        "{}:{:02}",
                        duration.num_hours(),
                        duration.num_minutes() % 60
                    ))
                    .size(60),
                )
                .style(style::TimerStyle {
                    is_running: self.is_running,
                    opacity: 1.0,
                }),
            )
            .push(Container::new(
                Container::new(Text::new(format!(":{:02}", duration.num_seconds() % 60)).size(60))
                    .style(style::TimerStyle {
                        is_running: self.is_running,
                        opacity: 0.5,
                    }),
            ));

        let start_stop_button = Container::new(
            Button::new(
                &mut self.start_stop_button,
                Container::new(match self.is_running {
                    true => Text::new("Pause"),
                    false => Text::new("Start"),
                })
                .center_x()
                .width(Length::Units(75)),
            )
            .on_press(Message::StartStopTimer)
            .style(style::ButtonStyle { foreground: None }),
        )
        .height(Length::Units(60))
        .center_y();

        let timer_operations = Container::new(
            Row::new()
                .push(Space::new(Length::Units(12), Length::Shrink))
                .push(Container::new(Text::new("Add")).padding(4))
                .push(
                    TextInput::new(
                        &mut self.time_text_input,
                        "all",
                        &self.time_input,
                        Message::TimeInputChanged,
                    )
                    .padding(3)
                    .width(Length::Units(50))
                    .style(style::TextInputStyle),
                )
                .push(Container::new(Text::new("to new entry with name")).padding(4))
                .push(
                    TextInput::new(
                        &mut self.name_text_input,
                        "",
                        &self.name_input,
                        Message::NameInputChanged,
                    )
                    .padding(3)
                    .style(style::TextInputStyle),
                )
                .push(Container::new(Text::new("or existing with number")).padding(4))
                .push(
                    TextInput::new(
                        &mut self.index_text_input,
                        "",
                        &self.index_input,
                        Message::IndexInputChanged,
                    )
                    .padding(3)
                    .width(Length::Units(25))
                    .style(style::TextInputStyle),
                )
                .push(Space::new(Length::Units(8), Length::Shrink))
                .push(
                    Button::new(
                        &mut self.apply_operation_button,
                        Row::new()
                            .push(Space::new(Length::Units(8), Length::Shrink))
                            .push(Text::new("Apply"))
                            .push(Space::new(Length::Units(8), Length::Shrink)),
                    )
                    .on_press(Message::ApplyOperation)
                    .padding(3)
                    .style(style::ButtonStyle { foreground: None }),
                )
                .push(Space::new(Length::Units(16), Length::Shrink)),
        )
        .width(Length::Fill)
        .center_x();

        let mut tracked_times_list = Column::new().spacing(6);

        for (i, tracked_time) in self.tracked_times.iter_mut().enumerate() {
            tracked_times_list = tracked_times_list.push(
                Container::new(
                    Row::new()
                        .push(
                            Container::new(
                                Row::new()
                                    .push(Space::new(Length::Units(4), Length::Shrink))
                                    .push(Text::new(format!("{}", i + 1)).size(28))
                                    .push(Space::new(Length::Units(4), Length::Shrink)),
                            )
                            .style(style::IndexStyle),
                        )
                        .push(Space::new(Length::Units(8), Length::Shrink))
                        .push(
                            Text::new(format!(
                                "{}:{:02}",
                                tracked_time.duration.num_hours(),
                                tracked_time.duration.num_minutes() % 60
                            ))
                            .size(28),
                        )
                        .push(Space::new(Length::Units(12), Length::Shrink))
                        .push(
                            Tooltip::new(
                                Text::new(&tracked_time.name)
                                    .size(28)
                                    .width(Length::Units(400)),
                                &tracked_time.name,
                                tooltip::Position::FollowCursor,
                            )
                            .style(style::TooltipStyle),
                        )
                        .push(Space::new(Length::Fill, Length::Shrink))
                        .push(
                            Button::new(
                                &mut tracked_time.copy_button,
                                Row::new()
                                    .push(Space::new(Length::Units(8), Length::Shrink))
                                    .push(Text::new("Copy"))
                                    .push(Space::new(Length::Units(8), Length::Shrink)),
                            )
                            .on_press(Message::CopyText(i))
                            .style(style::ButtonStyle { foreground: None }),
                        )
                        .push(Space::new(Length::Units(8), Length::Shrink))
                        .push(
                            Button::new(
                                &mut tracked_time.delete_button,
                                Row::new()
                                    .push(Space::new(Length::Units(8), Length::Shrink))
                                    .push(Text::new("Delete"))
                                    .push(Space::new(Length::Units(8), Length::Shrink)),
                            )
                            .on_press(Message::DeleteTrackedTime(i))
                            .style(style::ButtonStyle {
                                foreground: Some(Color::from_rgb8(0xc8, 0x40, 0x00)),
                            }),
                        )
                        .width(Length::Fill),
                )
                .padding(10)
                .style(style::TrackedTimeStyle),
            );
        }

        let tracked_times = Container::new(tracked_times_list).width(Length::Fill);

        Container::new(
            Column::new()
                .push(
                    Container::new(
                        Row::new()
                            .push(time)
                            .push(Space::new(Length::Units(12), Length::Shrink))
                            .push(start_stop_button),
                    )
                    .width(Length::Fill)
                    .center_x(),
                )
                .push(Space::new(Length::Fill, Length::Units(12)))
                .push(timer_operations)
                .push(Space::new(Length::Fill, Length::Units(20)))
                .push(
                    Scrollable::new(&mut self.tracked_times_scroll)
                        .push(tracked_times)
                        .width(Length::Fill),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .center_x()
        .into()
    }
}

mod style {
    use iced::{button, container, text_input, Background, Color};

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
}
