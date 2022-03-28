use iced::{button, Button, Column, Element, Row, Sandbox, Settings, Text};

/// App の状態を保持する State (Counter) を定義
/// この State (Counter) に Sandbox を実装することで App として実行することが可能
#[derive(Default)]
struct Counter {
    count: i32,
    increment_button_state: button::State, // Button の状態を保持する
    decrement_button_state: button::State,
}

/// ユーザの操作やイベントの通知に使う Message を定義
/// Debug, Clone の実装が必要
#[derive(Debug, Clone)]
enum Message {
    IncrementButtonPressed,
    DecrementButtonPressed,
}

/// Sandbox を State (Counter) に実装することで、App として実行が可能になる
impl Sandbox for Counter {
    /// State (Counter) とやりとりする関連型 Message を定義
    type Message = Message;

    /// State (Counter) を初期化 (iced 内部で使用される)
    fn new() -> Self {
        Self::default()
    }

    /// ウィンドウタイトルを設定
    fn title(&self) -> String {
        String::from("Counter")
    }

    /// Message を受け取って State (Counter) を更新する
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementButtonPressed => self.count += 1,
            Message::DecrementButtonPressed => self.count -= 1,
        }
    }

    /// State (Counter) に応じてウィジェットを表示する
    /// Into<Element<Message>> が実装されたウィジェットを返すことで画面に表示される
    /// Message を update() で処理した場合や Event 発生時のみ呼ばれる
    fn view(&mut self) -> Element<Message> {
        // 3つのウィジェットを作成
        let count_text = Text::new(self.count.to_string()).size(50);
        let increment_button = Button::new(&mut self.increment_button_state, Text::new("+"))
            .on_press(Message::IncrementButtonPressed);
        let decrement_button = Button::new(&mut self.decrement_button_state, Text::new("-"))
            .on_press(Message::DecrementButtonPressed);

        // 上記のウィジェットのうち2つの Button をRow にまとめる
        let button_row = Row::new().push(increment_button).push(decrement_button);

        // Text と Row にまとめた Button を Column にまとめる
        // into() によってウィジェットは Element<Message> に変換される
        Column::new().push(count_text).push(button_row).into()
    }
}

fn main() -> iced::Result {
    // Sandbox を実装した State (Counter) を実行する
    // Settings を変更すれば、ウィンドウサイズ等の設定が変更可能
    Counter::run(Settings::default())
}
