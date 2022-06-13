use yew::prelude::*;
use crate::utils::*;

#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
mod ij_mock;

mod utils;

enum Msg {
    AddOne,
}

struct Model {
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <VStack class="col">
                    <VStack class="tweet">
                        <HStack class="tweet-header">
                            <img class="tweet-author-icon" src="https://pbs.twimg.com/profile_images/1252746582815993856/sqWds6u9_bigger.jpg" />
                            <HStack class="account-info">
                                <b class="account-name">{"anatawa12 a.k.a. 翳河翔 プログラム言語ヲタク"}</b>
                                <span>{"@kakerigawa"}</span>
                            </HStack>
                        </HStack>
                        <div class="tweet-content">
                            {"apk: androidのアプリ"} <br/>
                            {"ipa: iOSのアプリ"} <br/>
                            {"app: apple/darwin系のアプリケーションバンドル(ディレクトリ)"}
                        </div>
                        <HStack>
                        </HStack>
                    </VStack>
                </VStack>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
