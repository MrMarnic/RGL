use crate::ui::ui_component::UIComponent;
use crate::ui::ui_text::UIText;

pub trait UIButton : UIComponent{
    fn get_state(&self) -> ButtonState;
    fn get_text(&self) -> &UIText;
    fn get_text_mut(&mut self) -> &mut UIText;
}

#[derive(PartialEq,Copy, Clone)]
pub enum ButtonState {
    NORMAL,
    HOVER,
    CLICK,
    PRESSED
}