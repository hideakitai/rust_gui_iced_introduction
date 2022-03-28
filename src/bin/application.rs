use iced::{button, executor, Application, Button, Clipboard, Command, Element, Settings, Text};

#[derive(Default)]
struct MyApplication {
    button_state: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonClicked,  // ボタンが押されたら 1 秒スリープし、
    AwakeFromSleep, // スリープから復帰したら AwakeFromSleep を送る
}

/// Application を State (AsyncHello) に実装することで、App として実行が可能になる
/// Application には非同期処理関連の型や Flags が Sandbox に追加されている
impl Application for MyApplication {
    /// 非同期処理の Executor を指定
    /// executor::Default は feature flag によって切り替わる
    /// - "tokio": executor::Tokio
    /// - "async-std": executor::AsyncStd
    /// - "smol": executor::Smol
    type Executor = executor::Default;
    /// 初期化を分岐するために使用する Flags だが、ここでは未使用
    type Flags = ();
    type Message = Message;

    /// State(AsyncHello) を初期化、Sandbox との違いは下記 2 点
    /// - Flags によって処理を分岐させることが可能
    /// - Command を使うことで、単発の非同期処理を行うことが可能
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("MyApplication")
    }

    /// Message を受け取って State (Counter) を更新する
    /// Command を使うことで、単発の非同期処理を行うことが可能
    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::ButtonClicked => {
                println!("Clicked!");

                // Command::perform() によって単発の非同期処理を行うことが可能
                // 第一引数に Future を与える (ここでは async fn を指定)
                // 第二引数に Future の返り値を引数とし Message を返す closure を指定 (Future 完了後に呼ばれる)
                Command::perform(sleep_for_a_second(), |_| Message::AwakeFromSleep)
            }
            Message::AwakeFromSleep => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        // 押されたら Message::ButtonClick を送るだけの Button を設置
        Button::new(&mut self.button_state, Text::new("click"))
            .on_press(Message::ButtonClicked)
            .into()
    }
}

/// Button が押されたら呼ばれる非同期関数
/// 1 秒スリープして文字列をコンソールに表示する
async fn sleep_for_a_second() {
    use async_std::task::sleep;
    use std::time::Duration;

    sleep(Duration::from_secs(1)).await;
    println!("Hello, from 1 sec sleep!");
}

fn main() -> iced::Result {
    MyApplication::run(Settings::default())
}
