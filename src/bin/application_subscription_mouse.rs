use iced::{executor, Application, Clipboard, Command, Element, Settings, Subscription, Text};
use iced_native::mouse::Button as MouseButton;

#[derive(Default)]
struct SubscriptionMouse;

#[derive(Debug, Clone)]
enum Message {
    MouseButtonPressed(MouseButton),
}

impl Application for SubscriptionMouse {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Subscription Mouse")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::MouseButtonPressed(button) => {
                println!("MouseButtonPressed: {button:?}");
            }
        }
        Command::none()
    }

    /// Subscription::batch() で複数の非同期処理 (Subscription) をまとめて登録することができる
    fn subscription(&self) -> Subscription<Message> {
        use iced_native::{mouse, subscription, Event};

        // マウスボタン入力があった場合、Message::MouseButtonPressed を送る
        subscription::events_with(|event, _status| match event {
            Event::Mouse(mouse::Event::ButtonPressed(button)) => {
                Some(Message::MouseButtonPressed(button))
            }
            _ => None,
        })
    }

    fn view(&mut self) -> Element<Message> {
        Text::new("").into()
    }
}

fn main() -> iced::Result {
    SubscriptionMouse::run(Settings::default())
}
