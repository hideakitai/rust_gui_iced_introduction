use iced::{button, Button, Element, Sandbox, Settings, Text};

/// App の状態を保持する State (Counter) を定義
/// この State に Sandbox を実装することで App として実行することが可能
#[derive(Default)]
struct MyButton {
    button_state: button::State, // Button の状態を保持する必要がある
}

/// ユーザの操作やイベントの通知に使う Message を定義
/// Debug, Clone の実装が必要
#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

/// Sandbox を State に実装することで、App として実行が可能になる
impl Sandbox for MyButton {
    /// State とやりとりする関連型 Message を定義
    type Message = Message;

    /// State を初期化 (iced 内部で使用される)
    fn new() -> Self {
        Self::default()
    }

    /// ウィンドウタイトルを設定
    fn title(&self) -> String {
        String::from("MyButton")
    }

    /// Message を受け取って State を更新する
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => println!("Button pressed"),
        }
    }

    /// State に応じてウィジェットを表示する
    /// Message を update() で処理した場合や Event 発生時のみ呼ばれる
    fn view(&mut self) -> Element<Message> {
        // Button ウィジェットを生成し、
        // Button が押されたら Message を送信する
        // このウィジェットを `into()` で Element<Element> に変換して返すことで描画される
        Button::new(&mut self.button_state, Text::new("Button"))
            .on_press(Message::ButtonPressed)
            .into()
    }
}

fn main() -> iced::Result {
    // Sandbox を実装した State (Counter) を実行する
    // Settings を変更すれば、ウィンドウサイズ等の設定が変更可能
    MyButton::run(Settings::default())
}
