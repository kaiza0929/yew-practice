/* この記述がコードの先頭(コメントを除く)にないとhtmlの属性を付けられない(コンパイルエラーになる) */
#![recursion_limit="256"]

use yew::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Model {
    link: ComponentLink<Self>, /* データバインディングに必要? */
    count: i32,
    keyword: String
}

/* 列挙型 どれかの値をとる */
pub enum Msg {
    Add,
    Reset,
    Input {keyword: String}
}

impl Component for Model {

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            count: 0,
            keyword: String::from("")
        }
    }

    fn view(&self) -> Html {
        /* htmlの記述を宣言 */
        html! {
            <div>
                <input type="button" class="btn btn-primary" value="カウントアップ" onclick=self.link.callback(|_| Msg::Add) />
                <input type="button" value="リセット" onclick=self.link.callback(|_| Msg::Reset) />
                /* 直接self.keywordを呼び出すとエラー(変数の束縛?) */
                <input type="text" value={self.keyword.clone()} oninput=self.link.callback(|e: InputData| Msg::Input {keyword: e.value}) />
                <p>{self.count}</p>
                <p>{self.keyword.clone()}</p>
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        /* reduxのreducerのようなもの */
        match msg {
            Msg::Add => self.count += 1,
            Msg::Reset => self.count = 0, 
            Msg::Input { keyword } => self.keyword = keyword
        }
        /* 返り値 */
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