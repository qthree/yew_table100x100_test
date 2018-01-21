use yew::prelude::*;
use Printer;

pub struct Square {
    class: &'static str,
    onsignal: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
    Clear
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub class: &'static str,
    pub onsignal: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            class: "square_red",
            onsignal: None,
        }
    }
}
use GetReciver;
impl<CTX: Printer + GetReciver<MSG = Msg> + 'static> Component<CTX> for Square{
    type Msg = Msg;
    type Properties = Props;

    fn create(_: &mut Env<CTX, Self>) -> Self {
        Square {
            class: "square_red",
            onsignal: None,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender {
        
        match msg {
            Msg::Clicked => {
                /*if let Some(ref mut callback) = self.onsignal {
                    callback.emit(());
                }*/
                self.class = if self.class == "square_red" {"square_green"} else {"square_red"};

                let send_back = context.send_back(|msg| msg);
                let reciver = context.get_reciver();
                reciver.send(Msg::Clear);
                reciver.set(send_back);
            },
            Msg::Clear => {
                self.class = "square_red"
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _context: &mut Env<CTX, Self>) -> ShouldRender {
        self.onsignal = props.onsignal;
        if self.class != props.class {
            self.class = props.class;
            true
        } else {
            false
        }        
    }
}

impl<CTX: Printer + GetReciver<MSG = Msg> + 'static> Renderable<CTX, Square> for Square {
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <td
                class=self.class,
                onclick=|_| Msg::Clicked,
            ></td>
        }
    }
}
