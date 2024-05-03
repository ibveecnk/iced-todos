use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Properties;
use iced::widget::{button, row, text, text_input, Column, Container, Scrollable};
use iced::{Alignment, Element, Length, Padding};
use log::info;

/// The path to the CSV file that stores the todos for persistence.
const DATA_PATH: &str = "./data.csv";

/// The state of the application.
pub struct Todos {
    /// List of todos in `String` format.
    data: Vec<String>,
    /// The text input for new todos. Synced with the input field via `Message::TextInputChanged`.
    text: String,
    /// Whether the todos have been modified since the last read/write operation.
    is_dirty: bool,
}

/// The messages that the application can send and receive.
#[derive(Debug, Clone, Hash)]
pub enum Message {
    /// Submit a new todo. Triggers adding the current text input `self.text` to the list of todos.
    SubmitTodo(),
    /// Remove a todo by its index in the list.
    RemoveTodo(usize),
    /// Update the text input for new todos.
    TextInputChanged(String),
}

/// Writes the given values to the CSV file at `DATA_PATH`.
fn write_to_csv(values: &[String]) -> anyhow::Result<()> {
    let mut writer = csv::Writer::from_path(DATA_PATH)?;

    values
        .iter()
        .map(|val| writer.write_record([val]))
        .all(|a| a.is_ok());
    Ok(())
}

/// Reads the values from the CSV file at `DATA_PATH`.
fn read_from_csv() -> anyhow::Result<Vec<String>> {
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(DATA_PATH)?;

    let data = reader
        .into_records()
        .flatten()
        .map(|v| v.get(0).unwrap_or("ERROR").to_string())
        .collect();

    Ok(data)
}

impl Default for Todos {
    fn default() -> Self {
        let csv_values = read_from_csv().unwrap_or_else(|_err| Vec::new());

        info!("Read from CSV: {:?}", csv_values);
        Self {
            data: csv_values,
            text: String::new(),
            is_dirty: false,
        }
    }
}

impl Todos {
    /// Updates the state of the application based on the given `Message`.
    pub(crate) fn update(&mut self, message: Message) {
        info!("Message: {:?}", message);
        match message {
            Message::SubmitTodo() => {
                let trimmed = self.text.trim();
                if !trimmed.is_empty() {
                    self.data.push(trimmed.to_string());
                    self.text.clear();

                    let write_result = write_to_csv(&self.data);
                    self.is_dirty = write_result.is_err();
                }
            }
            Message::RemoveTodo(i) => {
                self.data.remove(i);
            }
            Message::TextInputChanged(text) => {
                self.text = text;
                self.is_dirty = true;
            }
        }
    }

    /// Returns the variable title of the application.
    pub(crate) fn title(&self) -> String {
        if self.is_dirty {
            "Todos (*dirty*)".to_string()
        } else {
            "Todos".to_string()
        }
    }

    /// Returns the layout/view of the application.
    pub(crate) fn view(&self) -> Element<Message> {
        /// The spacing between elements in the UI.
        const SPACING: u16 = 10;

        /// The padding around the UI.
        const PADDING: f32 = 10.0;

        let input = row![
            text_input("New todo", self.text.as_str())
                .on_input(Message::TextInputChanged)
                .on_submit(Message::SubmitTodo()),
            button("Add").on_press(Message::SubmitTodo())
        ]
        .height(50)
        .align_items(Alignment::Center)
        .spacing(SPACING);

        let todos = self.data.iter().enumerate().map(|(i, v)| {
            row![
                text(v).width(Length::Fill),
                button("delete").on_press(Message::RemoveTodo(i))
            ]
            .spacing(SPACING)
            .padding([0, 20, 0, 0]) // Avoid overlap with scrollbar
            .align_items(Alignment::Center)
            .into()
        });

        let scrollable_todos = Scrollable::with_direction(
            Column::new().extend(todos).spacing(SPACING),
            iced::widget::scrollable::Direction::Vertical(Properties::default()),
        );

        let wrapper = Column::new()
            .push(input)
            .push(scrollable_todos)
            .spacing(SPACING);

        Container::new(wrapper)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(Padding::new(PADDING))
            .into()
    }
}
