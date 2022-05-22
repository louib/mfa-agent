use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct NumPad {}

#[derive(Clone, Debug)]
pub enum Message {
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
        }
    }

    fn view(&self) -> VNode<NumPad> {
        gtk! {
            <Application::new_unwrap(Some(crate::consts::APP_ID), ApplicationFlags::empty())>
                <Window border_width=20 on destroy=|_| Message::Exit>
                    <Label label="{{project-name}}" />
                </Window>
            </Application>
        }
    }
}
