use iced::{executor, Application, Clipboard, Command, Element, Settings, Subscription, Text};
use iced_futures::futures;
use iced_native::subscription::Recipe;
use std::time::{Duration, Instant};

/// 独自の Subscription を実装する型
pub struct Timer;

/// Recipe<Hasher, Event> を実装することで、独自の Subscription が構築できる
impl<Hasher, Event> Recipe<Hasher, Event> for Timer
where
    Hasher: std::hash::Hasher,
{
    /// Subscription の Output 型を定義する
    type Output = Instant;

    /// Subscription を比較するための hash メソッド
    fn hash(&self, state: &mut Hasher) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }

    /// Recipe を実行して Subscription Event を生成する
    /// Subscription Event は Stream として返される
    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, Event>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;
        async_std::stream::interval(Duration::from_secs(1))
            .map(|_| Instant::now())
            .boxed()
    }
}

#[derive(Default)]
struct SubscriptionCustomRecipe;

#[derive(Debug, Clone)]
enum Message {
    Tick(Instant),
}

impl Application for SubscriptionCustomRecipe {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Subscription Custom Recipe")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::Tick(instant) => {
                println!("Tick {instant:?}");
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        // Recipe を実装した Timer を生成し、
        // Subscription::from_recipe で Subscription Stream を生成する
        iced::Subscription::from_recipe(Timer).map(Message::Tick)
    }

    fn view(&mut self) -> Element<Message> {
        Text::new("").into()
    }
}

fn main() -> iced::Result {
    SubscriptionCustomRecipe::run(Settings::default())
}
