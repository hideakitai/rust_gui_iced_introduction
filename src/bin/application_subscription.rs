use iced::{executor, Application, Clipboard, Command, Element, Settings, Subscription, Text};

#[derive(Default)]
struct ApplicationSubscription;

#[derive(Debug, Clone)]
enum Message {
    Tick, // 1 秒ごとに非同期に Message が来る
}

impl Application for ApplicationSubscription {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Application Subscription")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::Tick => {
                println!("Tick");
                Command::none()
            }
        }
    }

    /// 定期的な非同期処理を行うために、デフォルト実装をオーバーライドする
    fn subscription(&self) -> Subscription<Message> {
        use std::time::Duration;

        // 1 秒ごとに Message::Tick を Subscription<Message> として返す
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn view(&mut self) -> Element<Message> {
        Text::new("").into()
    }
}

fn main() -> iced::Result {
    ApplicationSubscription::run(Settings::default())
}
