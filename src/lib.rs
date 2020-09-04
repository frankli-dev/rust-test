use seed::prelude::*;
use seed::*;

#[derive(Default)]
struct Model {
    text: String,
    items: Vec<String>,
}

#[derive(Clone)]
enum Msg {
    ChangeText(String),
    AddItem,
    ClearItems,
    RemoveItem(usize),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        ChangeText(new_text) => model.text = new_text,
        AddItem => {
            if model.text.len() > 0 {
                model.items.push(model.text.clone());
                model.text.clear();
            }
        }
        ClearItems => model.items.clear(),
        RemoveItem(index) => {
            model.items.remove(index);
        }
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        img![
            C!["logo"],
            attrs! {At::Src => "https://cameras.liveviewtech.com/img/LVLogo_small.png"}
        ],
        div![
            C!["main"],
            input![
                attrs! {
                    At::Placeholder => "Enter some text...",
                    At::Value => &model.text
                },
                input_ev(Ev::Input, Msg::ChangeText),
            ],
            div![
                C!["buttons"],
                button![C!["btn-save"], "Save", ev(Ev::Click, move |_| Msg::AddItem)],
                button![
                    C!["btn-clear"],
                    "Clear",
                    ev(Ev::Click, move |_| Msg::ClearItems)
                ],
            ],
            ul![
                C!["items-list"],
                model.items.iter().enumerate().map(|(index, item)| {
                    li![
                        item,
                        a!["X", ev(Ev::Click, move |_| Msg::RemoveItem(index))]
                    ]
                })
            ]
        ],
    ]
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
