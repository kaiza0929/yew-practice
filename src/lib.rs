use yew::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    count: i32
}

/* 列挙型 どちらかの値をとる */
pub enum Msg {
    ADD,
    RESET
}

impl Component for Model {

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            count: 0
        }
    }

    fn view(&self) -> Html {
        /* htmlの記述を宣言 */
        html! {
            <div>
                <input type="button" value="カウントアップ" onclick=self.link.callback(|_| Msg::ADD) />
                <input type="button" value="リセット" onclick=self.link.callback(|_| Msg::RESET) />
                <p>{self.count}</p>
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ADD => self.count += 1, /* msg == ADDの場合 */
            Msg::RESET => self.count = 0 /* msg == RESETの場合 */
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

}

#[wasm_bindgen(start)]
pub fn run() {
    App::<Model>::new().mount_to_body();
}