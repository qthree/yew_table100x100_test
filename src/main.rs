#[macro_use]
extern crate yew;
use yew::prelude::*;

mod square;
use square::Square;

use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
}


pub trait Printer {
    fn print(&mut self, data: &str);
}

impl Printer for Context {
    fn print(&mut self, data: &str) {
        self.console.log(data);
    }
}

struct Model {
    selected: Option<(u32, u32)>
}

enum Msg {
    Select(u32, u32),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: &mut Env<Context, Self>) -> Self {
        Model {
            selected: None
        }
    }

    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Select(x, y) => {
                self.selected = Some((x, y));
            }
        }
        true
    }
}

fn square_class(this: (u32, u32), selected: Option<(u32, u32)>) -> &'static str {
    match selected {
        Some(xy) if xy == this => {"square_green"},
        _ => {"square_red"}
    }
}

fn view_square(selected: Option<(u32, u32)>, row: u32, column: u32) -> Html<Context, Model> {
    /*html! {
        <td
            class=square_class((column, row), selected),
            onclick=move |_| Msg::Select(column, row),
        >
        </td>
    }*/
    html! {
        <Square:
            onsignal=move |_| Msg::Select(column, row),
            class=square_class((column, row), selected),
        />
    }
}

fn view_row(selected: Option<(u32, u32)>, row: u32) -> Html<Context, Model> {
    html! {
        <tr>
            {for (0..99).map(|column| {
                view_square(selected, row, column)
            })}
        </tr>
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <table>
                {for (0..99).map(|row| {
                    view_row(self.selected, row)
                })}
            </table>
        }        
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}