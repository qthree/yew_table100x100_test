use yew::prelude::*;
use Printer;
use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
struct Stat {
    vieved: u32,
    updated: u32,
    changed: u32
}

pub struct Square {
    class: &'static str,
    onsignal: Option<Callback<()>>,
    stat: Cell<Stat>,
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
impl<CTX: Printer + GetReciver<CTX = CTX, COMP = Self> + 'static> Component<CTX> for Square{
    type Msg = Msg;
    type Properties = Props;

    fn create(_: &mut Env<CTX, Self>) -> Self {
        Square {
            class: "square_red",
            onsignal: None,
            stat: Cell::new(Stat {
                    vieved: 0,
                    updated: 0,
                    changed: 0
                })
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender {
        /*{
            let mut stat = self.stat.get();
            stat.updated+=1;
            context.print(&format!("{:?}", stat));
            self.stat.set(stat);
        }*/
        
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

    fn change(&mut self, props: Self::Properties, context: &mut Env<CTX, Self>) -> ShouldRender {
        /*{
            let mut stat = self.stat.get();
            stat.changed+=1;
            context.print(&format!("{:?}", stat));
            self.stat.set(stat);
        }*/
        self.onsignal = props.onsignal;
        if self.class != props.class {
            self.class = props.class;
            true
        } else {
            false
        }        
    }
}

impl<CTX: Printer + GetReciver<CTX = CTX, COMP = Self> + 'static> Renderable<CTX, Square> for Square {
    fn view(&self) -> Html<CTX, Self> {
        /*{
            let mut stat = self.stat.get();
            stat.vieved+=1;
            self.stat.set(stat);
        }*/
        html! {
            <td
                class=self.class,
                onclick=|_| Msg::Clicked,
            ></td>
        }
    }
}
