use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct NumPad {}

#[derive(Clone, Debug)]
pub enum Message {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Exit,
}

impl Component for NumPad {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            _ => UpdateAction::None,
        }
    }

    fn view(&self) -> VNode<NumPad> {
        gtk! {
            <Application::new_unwrap(Some(crate::consts::APP_ID), ApplicationFlags::empty())>
                <Window border_width=20 on destroy=|_| Message::Exit>
                    <Label label="{{project-name}}" />
                    <Grid visible=true hexpand=false row_spacing=6 column_spacing=6 column_homogeneous=true row_homogeneous=true>
                      <Button visible=true focus_on_click=false>
                        <Label label="1" width_chars=1/>
                      </Button>
                      <Button visible=true focus_on_click=false>
                        <Label label="2" width_chars=1/>
                      </Button>
                      <Button visible=true focus_on_click=false>
                        <Label label="3" width_chars=1/>
                      </Button>
                      <Button visible=true focus_on_click=false>
                        <Label label="4" width_chars=1/>
                      </Button>
                    </Grid>
                </Window>
            </Application>
        }
    }
}
