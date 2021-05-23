#![windows_subsystem = "windows"]
mod style;

use iced::{
    button, executor, scrollable, text_input, time, tooltip, window, Application, Button,
    Clipboard, Color, Column, Command, Container, Element, Length, Row, Rule, Scrollable, Settings,
    Space, Subscription, Text, TextInput, Tooltip,
};
use rusqlite::{params, Connection};

pub fn main() -> iced::Result {
    SimpleTimeTracker::run(Settings {
        window: window::Settings {
            size: (700, 400),
            min_size: Some((700, 400)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct SimpleTimeTracker {
    is_dark_mode: bool,
    is_running: bool,
    start_time: chrono::DateTime<chrono::Local>,
    pause_time: chrono::DateTime<chrono::Local>,
    tracked_times: Vec<TrackedTime>,

    start_stop_button: button::State,
    dark_mode_button: button::State,
    time_text_input: text_input::State,
    time_input: String,
    description_text_input: text_input::State,
    description_input: String,
    index_text_input: text_input::State,
    index_input: String,
    apply_operation_button: button::State,
    tracked_times_scroll: scrollable::State,
}

#[derive(Debug, Clone)]
struct TrackedTime {
    description: String,
    duration: chrono::Duration,

    copy_button: button::State,
    delete_button: button::State,
}

impl TrackedTime {
    fn new(duration: chrono::Duration, description: String) -> Self {
        TrackedTime {
            description,
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
    DarkModeToggle,
    TimeInputChanged(String),
    DescriptionInputChanged(String),
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
            let minutes = if parts.len() == 2 {
                parts[1].parse()
            } else {
                parts[0].parse()
            };
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
                let hours = parts[0].parse();
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

        // Ensure only either description or index is set
        if (self.description_input.len() > 0) == (self.index_input.len() > 0) {
            return;
        }

        if self.description_input.len() > 0 {
            self.tracked_times
                .push(TrackedTime::new(duration, self.description_input.clone()));
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
        self.description_input.clear();
        self.index_input.clear();
        if self.is_running {
            self.start_time = self.start_time + duration;
        } else {
            self.pause_time = self.pause_time - duration;
        }
    }

    fn write_to_db(&self) {
        let db = Connection::open("simple_time_tracker.sqlite").unwrap();

        db.execute("DELETE FROM TrackedTimes", []).unwrap();
        let mut stmt = db
            .prepare("INSERT INTO TrackedTimes (Seconds, Description) VALUES (?1, ?2)")
            .unwrap();
        for tracked_time in self.tracked_times.iter() {
            stmt.execute(params![
                tracked_time.duration.num_seconds(),
                tracked_time.description
            ])
            .unwrap();
        }
    }
}

impl Application for SimpleTimeTracker {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let db = Connection::open("simple_time_tracker.sqlite").unwrap();

        db.execute(
            "CREATE TABLE IF NOT EXISTS TrackedTimes (
                ID	INTEGER PRIMARY KEY,
                Seconds	INTEGER NOT NULL,
                Description	TEXT NOT NULL
        )",
            [],
        )
        .unwrap();

        let mut stmt = db
            .prepare("SELECT Seconds, Description FROM TrackedTimes")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();

        let mut tracked_times = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            tracked_times.push(TrackedTime::new(
                chrono::Duration::seconds(row.get(0).unwrap()),
                row.get(1).unwrap(),
            ));
        }

        (
            Self {
                is_dark_mode: true,
                is_running: false,
                start_time: chrono::Local::now(),
                pause_time: chrono::Local::now(),
                tracked_times: tracked_times,

                start_stop_button: button::State::new(),
                dark_mode_button: button::State::new(),
                time_text_input: text_input::State::new(),
                time_input: String::new(),
                description_text_input: text_input::State::new(),
                description_input: String::new(),
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
            Message::DarkModeToggle => self.is_dark_mode = !self.is_dark_mode,
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
            Message::DescriptionInputChanged(input) => self.description_input = input,
            Message::IndexInputChanged(input) => {
                if input.len() == 0 || (input.len() < 3 && input.parse::<usize>().is_ok()) {
                    self.index_input = input
                }
            }
            Message::ApplyOperation => {
                self.apply_operation();
                self.write_to_db();
            }
            Message::DeleteTrackedTime(i) => {
                self.tracked_times.remove(i);
                self.write_to_db();
            }
            Message::CopyText(i) => {
                clipboard.write(self.tracked_times[i].description.clone());
                self.write_to_db();
            }
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
                    is_dark_mode: self.is_dark_mode,
                    is_running: self.is_running,
                    opacity: 1.0,
                }),
            )
            .push(Container::new(
                Container::new(Text::new(format!(":{:02}", duration.num_seconds() % 60)).size(60))
                    .style(style::TimerStyle {
                        is_dark_mode: self.is_dark_mode,
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
            .style(style::ButtonStyle {
                is_dark_mode: self.is_dark_mode,
                foreground: None,
            }),
        )
        .height(Length::Units(60))
        .center_y();

        let dark_mode_button = Container::new(
            Button::new(
                &mut self.dark_mode_button,
                Container::new(match self.is_dark_mode {
                    true => Text::new("Light"),
                    false => Text::new("Dark"),
                })
                .center_x()
                .width(Length::Units(75)),
            )
            .on_press(Message::DarkModeToggle)
            .style(style::ButtonStyle {
                is_dark_mode: self.is_dark_mode,
                foreground: None,
            }),
        )
        .height(Length::Units(60))
        .center_y();

        let timer_operations = Container::new(
            Row::new()
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Container::new(Text::new("Add "))
                                .height(Length::Fill)
                                .center_y()
                                .style(style::TextStyle {
                                    is_dark_mode: self.is_dark_mode,
                                }),
                        )
                        .push(
                            Container::new(
                                TextInput::new(
                                    &mut self.time_text_input,
                                    "all",
                                    &self.time_input,
                                    Message::TimeInputChanged,
                                )
                                .padding(3)
                                .width(Length::Units(50))
                                .style(style::TextInputStyle {
                                    is_dark_mode: self.is_dark_mode,
                                }),
                            )
                            .height(Length::Fill)
                            .center_y(),
                        )
                        .push(Space::with_width(Length::Units(16)))
                        .width(Length::FillPortion(1)),
                )
                .push(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    Container::new(Text::new("to new entry called "))
                                        .height(Length::Fill)
                                        .center_y()
                                        .style(style::TextStyle {
                                            is_dark_mode: self.is_dark_mode,
                                        }),
                                )
                                .push(
                                    Container::new(
                                        TextInput::new(
                                            &mut self.description_text_input,
                                            "description",
                                            &self.description_input,
                                            Message::DescriptionInputChanged,
                                        )
                                        .padding(3)
                                        .style(
                                            style::TextInputStyle {
                                                is_dark_mode: self.is_dark_mode,
                                            },
                                        ),
                                    )
                                    .height(Length::Fill)
                                    .width(Length::Fill)
                                    .center_y(),
                                )
                                .height(Length::FillPortion(3)),
                        )
                        .push(
                            Row::new()
                                .push(Rule::horizontal(8).style(style::RuleStyle))
                                .push(
                                    Container::new(Text::new("or"))
                                        .height(Length::Fill)
                                        .center_y()
                                        .style(style::TextStyle {
                                            is_dark_mode: self.is_dark_mode,
                                        }),
                                )
                                .push(Rule::horizontal(8).style(style::RuleStyle))
                                .height(Length::FillPortion(2)),
                        )
                        .push(
                            Row::new()
                                .push(
                                    Container::new(Text::new("existing entry with number "))
                                        .height(Length::Fill)
                                        .center_y()
                                        .style(style::TextStyle {
                                            is_dark_mode: self.is_dark_mode,
                                        }),
                                )
                                .push(
                                    Container::new(
                                        TextInput::new(
                                            &mut self.index_text_input,
                                            "#",
                                            &self.index_input,
                                            Message::IndexInputChanged,
                                        )
                                        .padding(3)
                                        .width(Length::Units(30))
                                        .style(
                                            style::TextInputStyle {
                                                is_dark_mode: self.is_dark_mode,
                                            },
                                        ),
                                    )
                                    .height(Length::Fill)
                                    .center_y(),
                                )
                                .height(Length::FillPortion(3)),
                        )
                        .width(Length::FillPortion(2)),
                )
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Units(24)))
                        .push(
                            Container::new(
                                Button::new(
                                    &mut self.apply_operation_button,
                                    Row::new()
                                        .push(Space::with_width(Length::Units(12)))
                                        .push(Text::new("Apply"))
                                        .push(Space::with_width(Length::Units(12))),
                                )
                                .on_press(Message::ApplyOperation)
                                .padding(3)
                                .style(style::ButtonStyle {
                                    is_dark_mode: self.is_dark_mode,
                                    foreground: None,
                                }),
                            )
                            .height(Length::Fill)
                            .center_y(),
                        )
                        .push(Space::with_width(Length::Fill))
                        .width(Length::FillPortion(1)),
                )
                .height(Length::Units(80)),
        )
        .padding(8)
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
                                    .push(Space::with_width(Length::Units(4)))
                                    .push(Text::new(format!("{}", i + 1)).size(28))
                                    .push(Space::with_width(Length::Units(4))),
                            )
                            .height(Length::Fill)
                            .width(Length::Units(50))
                            .center_x()
                            .center_y()
                            .style(style::IndexStyle {
                                is_dark_mode: self.is_dark_mode,
                            }),
                        )
                        .push(Space::with_width(Length::Units(8)))
                        .push(
                            Container::new(
                                Text::new(format!(
                                    "{}:{:02}",
                                    tracked_time.duration.num_hours(),
                                    tracked_time.duration.num_minutes() % 60
                                ))
                                .size(28),
                            )
                            .height(Length::Fill)
                            .center_y()
                            .style(style::TextStyle {
                                is_dark_mode: self.is_dark_mode,
                            }),
                        )
                        .push(Space::with_width(Length::Units(12)))
                        .push(
                            Container::new(
                                Tooltip::new(
                                    Text::new(&tracked_time.description)
                                        .size(24)
                                        .width(Length::Fill),
                                    &tracked_time.description,
                                    tooltip::Position::FollowCursor,
                                )
                                .style(style::TooltipStyle),
                            )
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .center_y()
                            .style(style::TextStyle {
                                is_dark_mode: self.is_dark_mode,
                            }),
                        )
                        .push(
                            Container::new(
                                Button::new(
                                    &mut tracked_time.copy_button,
                                    Row::new()
                                        .push(Space::with_width(Length::Units(8)))
                                        .push(Text::new("Copy Text"))
                                        .push(Space::with_width(Length::Units(8))),
                                )
                                .on_press(Message::CopyText(i))
                                .width(Length::Shrink)
                                .style(style::ButtonStyle {
                                    is_dark_mode: self.is_dark_mode,
                                    foreground: None,
                                }),
                            )
                            .height(Length::Fill)
                            .center_y(),
                        )
                        .push(Space::with_width(Length::Units(8)))
                        .push(
                            Container::new(
                                Button::new(
                                    &mut tracked_time.delete_button,
                                    Row::new()
                                        .push(Space::with_width(Length::Units(8)))
                                        .push(Text::new("Delete"))
                                        .push(Space::with_width(Length::Units(8))),
                                )
                                .on_press(Message::DeleteTrackedTime(i))
                                .width(Length::Shrink)
                                .style(style::ButtonStyle {
                                    is_dark_mode: self.is_dark_mode,
                                    foreground: Color::from_rgb8(0xc8, 0x40, 0x00).into(),
                                }),
                            )
                            .height(Length::Fill)
                            .center_y(),
                        )
                        .push(Space::with_width(Length::Units(8)))
                        .width(Length::Fill),
                )
                .height(Length::Units(50))
                .style(style::TrackedTimeStyle {
                    is_dark_mode: self.is_dark_mode,
                }),
            );
        }

        let tracked_times = Container::new(tracked_times_list).width(Length::Fill);

        Container::new(
            Column::new()
                .push(
                    Container::new(
                        Row::new()
                            .push(time)
                            .push(Space::with_width(Length::Units(12)))
                            .push(start_stop_button)
                            .push(Space::with_width(Length::Units(8)))
                            .push(dark_mode_button),
                    )
                    .width(Length::Fill)
                    .center_x(),
                )
                .push(Space::with_height(Length::Units(12)))
                .push(timer_operations)
                .push(Space::with_height(Length::Units(20)))
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
        .style(style::RootStyle {
            is_dark_mode: self.is_dark_mode,
        })
        .into()
    }
}

const _SUN_SVG: &'static str = "<svg viewBox=\"0 0 457.32 457.32\" height=\"10\">
        <path d=\"M228.66,112.692c-63.945,0-115.968,52.022-115.968,115.967c0,63.945,52.023,115.968,115.968,115.968
            s115.968-52.023,115.968-115.968C344.628,164.715,292.605,112.692,228.66,112.692z\"/>
        <path d=\"M401.429,228.66l42.467-57.07c2.903-3.9,3.734-8.966,2.232-13.59c-1.503-4.624-5.153-8.233-9.794-9.683
            l-67.901-21.209l0.811-71.132c0.056-4.862-2.249-9.449-6.182-12.307c-3.934-2.858-9.009-3.633-13.615-2.077l-67.399,22.753
            L240.895,6.322C238.082,2.356,233.522,0,228.66,0c-4.862,0-9.422,2.356-12.235,6.322l-41.154,58.024l-67.4-22.753
            c-4.607-1.555-9.682-0.781-13.615,2.077c-3.933,2.858-6.238,7.445-6.182,12.307l0.812,71.132l-67.901,21.209
            c-4.641,1.45-8.291,5.059-9.793,9.683c-1.503,4.624-0.671,9.689,2.232,13.59l42.467,57.07l-42.467,57.07
            c-2.903,3.9-3.734,8.966-2.232,13.59c1.502,4.624,5.153,8.233,9.793,9.683l67.901,21.208l-0.812,71.132
            c-0.056,4.862,2.249,9.449,6.182,12.307c3.934,2.857,9.007,3.632,13.615,2.077l67.4-22.753l41.154,58.024
            c2.813,3.966,7.373,6.322,12.235,6.322c4.862,0,9.422-2.356,12.235-6.322l41.154-58.024l67.399,22.753
            c4.606,1.555,9.681,0.781,13.615-2.077c3.933-2.858,6.238-7.445,6.182-12.306l-0.811-71.133l67.901-21.208
            c4.641-1.45,8.291-5.059,9.794-9.683c1.502-4.624,0.671-9.689-2.232-13.59L401.429,228.66z M228.66,374.627
            c-80.487,0-145.968-65.481-145.968-145.968S148.173,82.692,228.66,82.692s145.968,65.48,145.968,145.967
            S309.147,374.627,228.66,374.627z\"/>
    </svg>";
