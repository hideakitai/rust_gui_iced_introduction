use iced::{executor, Application, Clipboard, Command, Element, Settings, Subscription, Text};
use iced_native::{keyboard::KeyCode, mouse::Button as MouseButton};

#[derive(Default)]
struct MultipleSubscription;

#[derive(Debug, Clone)]
enum Message {
    Tick,
    KeyPressed(KeyCode),
    MouseButtonPressed(MouseButton),
}

impl Application for MultipleSubscription {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Multiple Subscription")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::Tick => {
                println!("Tick");
            }
            Message::KeyPressed(key) => {
                println!("KeyPressed: {key:?}");
            }
            Message::MouseButtonPressed(button) => {
                println!("MouseButtonPressed: {button:?}");
            }
        }
        Command::none()
    }

    /// Subscription::batch() で複数の非同期処理 (Subscription) をまとめて登録することができる
    fn subscription(&self) -> Subscription<Message> {
        use iced_native::{keyboard, mouse, subscription, Event};
        use std::time::Duration;

        // 1 秒ごとに Message::Tick を送る
        let tick = iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick);

        // キーボード入力があった場合、Message::KeyPressed を送る
        let key_pressed = subscription::events_with(|event, _status| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code,
                modifiers: _,
            }) => Some(Message::KeyPressed(key_code)),
            _ => None,
        });

        // マウスボタン入力があった場合、Message::MouseButtonPressed を送る
        let mouse_pressed = subscription::events_with(|event, _status| match event {
            Event::Mouse(mouse::Event::ButtonPressed(button)) => {
                Some(Message::MouseButtonPressed(button))
            }
            _ => None,
        });

        // Subscription::batch() に複数の Subscription を渡す
        Subscription::batch([tick, key_pressed, mouse_pressed])
    }

    fn view(&mut self) -> Element<Message> {
        Text::new("").into()
    }
}

fn main() -> iced::Result {
    MultipleSubscription::run(Settings::default())
}
