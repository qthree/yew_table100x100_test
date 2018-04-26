#[macro_use]
extern crate yew;
use yew::prelude::*;

type Context = ();

type Item = u32;

const SIZE: usize = 100;

#[derive(Default)]
struct Model {
    selected: Option<(usize, usize)>,
    data: Vec<Vec<Item>>,
    text: String
}

enum Msg {
    Populate,
    Select(usize, usize),
    Text(String),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: (), _: &mut Env<Context, Self>) -> Self {
        Default::default()
    }

    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Populate => {
                self.data = (0..SIZE).map(|_row| (0..SIZE).map(|_column| 0).collect()).collect();
            },
            Msg::Select(x, y) => {
                self.selected = Some((x, y));
            }
            Msg::Text(text) => {
                self.text = text;
            }
        }
        true
    }
}

fn _square_class(this: (usize, usize), selected: Option<(usize, usize)>) -> &'static str {
    match selected {
        Some(xy) if xy == this => {"square_green"},
        _ => {"square_red"}
    }
}

fn square_style(this: (usize, usize), selected: Option<(usize, usize)>) -> String {
    let value = match selected {
        Some((y, x)) => {
            let dif = ( (this.0 as i32 - y as i32), (this.1 as i32 - x as i32) );
            255 - u32::min(255, (4.0*(dif.0*dif.0 + dif.1*dif.1) as f32).sqrt().round() as u32)
        },
        _ => 0
    };
    format!("background-color: rgb({}, {}, 150)", value, value)
}

fn view_square(selected: Option<(usize, usize)>, row_index: usize, column_index: usize, _item: &Item) -> Html<Context, Model> {
    html! {
        <td
            /*class=square_class((column_index, row_index), selected),*/
            style=square_style((row_index, column_index), selected),
            onclick=move |_| Msg::Select(row_index, column_index),
        >
        </td>
    }
}

fn view_row(selected: Option<(usize, usize)>, row_index: usize, row: &[Item]) -> Html<Context, Model> {
    html! {
        <tr>
            {for row.iter().enumerate().map(|(column_index, item)| {
                view_square(selected, row_index, column_index, item)
            })}
        </tr>
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <p><button 
                    onclick=move |_| Msg::Populate,
                >
                    {"Populate"}
                </button></p>
                <p><input
                    placeholder="Input text here",
                    oninput=move |e: InputData| Msg::Text(e.value),
                >
                </input></p><p>
                   {&self.text}
                </p>
                <table>
                    {for self.data.iter().enumerate().map(|(row_index, row)| {
                        view_row(self.selected, row_index, row)
                    })}
                </table>
            </div>
        }        
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}