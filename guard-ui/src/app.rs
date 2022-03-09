use yew::{Component, Context, Html, html};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {{ "Menu" }}
            </div>
        }
    }
}
