use iced::{executor, Application, Clipboard, Command, Element, Settings, Subscription, Text};
use iced_native::keyboard::KeyCode;

#[derive(Default)]
struct SubscriptionKeyboard;

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(KeyCode),
}

impl Application for SubscriptionKeyboard {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Subscription Keyboard")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::KeyPressed(key) => {
                println!("KeyPressed: {key:?}");
            }
        }
        Command::none()
    }

    /// Subscription::batch() で複数の非同期処理 (Subscription) をまとめて登録することができる
    fn subscription(&self) -> Subscription<Message> {
        use iced_native::{keyboard, subscription, Event};

        // キーボード入力があった場合、Message::KeyPressed を送る
        subscription::events_with(|event, _status| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code,
                modifiers: _,
            }) => Some(Message::KeyPressed(key_code)),
            _ => None,
        })
    }

    fn view(&mut self) -> Element<Message> {
        Text::new("").into()
    }
}

fn main() -> iced::Result {
    SubscriptionKeyboard::run(Settings::default())
}
