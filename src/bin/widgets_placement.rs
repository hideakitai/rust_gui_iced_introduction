use iced::{
    button, Align, Button, Color, Column, Container, Element, Length, Row, Sandbox, Settings, Text,
};

#[derive(Default)]
struct Counter {
    count: i32,
    increment_button_state: button::State, // Button の状態を保持する
    decrement_button_state: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementButtonPressed,
    DecrementButtonPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementButtonPressed => self.count += 1,
            Message::DecrementButtonPressed => self.count -= 1,
        }
    }

    fn view(&mut self) -> Element<Message> {
        // 3つのウィジェットを作成
        // color, size を指定
        let count_text = Text::new(self.count.to_string())
            .color(Color::from_rgb(0.0, 0.0, 1.0))
            .size(50);
        // padding を指定
        let increment_button = Button::new(&mut self.increment_button_state, Text::new("+"))
            .padding(10)
            .on_press(Message::IncrementButtonPressed);
        // padding を指定
        let decrement_button = Button::new(&mut self.decrement_button_state, Text::new("-"))
            .padding(10)
            .on_press(Message::DecrementButtonPressed);

        // 上記のウィジェットのうち2つの Button をRow にまとめる
        // padding, spacing, max_width, align を設定する
        let button_row = Row::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Align::Center)
            .push(increment_button)
            .push(decrement_button);

        // Text と Row にまとめた Button を Column にまとめる
        // padding, spacing, max_width, align を設定する
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Align::Center)
            .push(count_text)
            .push(button_row);

        // Column にまとめたすべての要素を Container にまとめ、
        // Window の width, height を設定し、
        // Window の上下の中央に配置する
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    // Settings を使って、ウィンドウサイズを設定する
    let mut settings = Settings::default();
    settings.window.size = (200, 200);
    Counter::run(settings)
}
