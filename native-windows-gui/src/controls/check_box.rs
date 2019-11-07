use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, BS_AUTOCHECKBOX, BS_AUTO3STATE};
use crate::win32::window_helper as wh;
use crate::{Font, SystemError};
use super::{ControlBase, ControlHandle};

const NOT_BOUND: &'static str = "CheckBox is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: CheckBox handle is not HWND!";


bitflags! {
    pub struct CheckBoxFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TRISTATE = BS_AUTO3STATE;
    }
}

/// Represents the check status of a checkbox
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CheckBoxState {
    Checked,
    Unchecked,

    /// New state for the tristate checkbox
    Indeterminate
}

/**
A check box consists of a square box and an application-defined label, icon, or bitmap that indicates a choice the user can make by selecting the button.
Applications typically display check boxes to enable the user to choose one or more options that are not mutually exclusive.

Note: Internally, check box are `Button` and as such, they trigger the same events
*/
#[derive(Default, Debug)]
pub struct CheckBox {
    pub handle: ControlHandle
}

impl CheckBox {

    pub fn builder<'a>() -> CheckBoxBuilder<'a> {
        CheckBoxBuilder {
            text: "A checkbox",
            size: (100, 25),
            position: (0, 0),
            background_color: None,
            check_state: CheckBoxState::Unchecked,
            flags: None,
            font: None,
            parent: None
        }
    }

    /// Return `true` if the checkbox can have a third state or `false` otherwise
    pub fn tristate(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let style = wh::get_style(handle);

        style & BS_AUTO3STATE == BS_AUTO3STATE
    }

    /// Sets or unsets the checkbox as tristate
    pub fn set_tristate(&self, tri: bool) {
        use winapi::um::winuser::{BM_SETSTYLE};
        use winapi::shared::minwindef::WPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        
        let style = match tri {
            true => BS_AUTO3STATE,
            false => BS_AUTOCHECKBOX
        };

        wh::send_message(handle, BM_SETSTYLE, style as WPARAM, 1);
    }

    /// Return the check state of the check box
    pub fn check_state(&self) -> CheckBoxState {
        use winapi::um::winuser::{BM_GETCHECK, BST_CHECKED, BST_INDETERMINATE, BST_UNCHECKED};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        match wh::send_message(handle, BM_GETCHECK, 0, 0) as usize {
            BST_UNCHECKED => CheckBoxState::Unchecked,
            BST_CHECKED => CheckBoxState::Checked,
            BST_INDETERMINATE => CheckBoxState::Indeterminate,
            _ => unreachable!()
        }
    }

    /// Sets the check state of the check box
    pub fn set_check_state(&self, state: CheckBoxState) {
        use winapi::um::winuser::{BM_SETCHECK, BST_CHECKED, BST_INDETERMINATE, BST_UNCHECKED};
        use winapi::shared::minwindef::WPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let x = match state {
            CheckBoxState::Unchecked => BST_UNCHECKED,
            CheckBoxState::Checked => BST_CHECKED,
            CheckBoxState::Indeterminate => BST_INDETERMINATE,
        };

        wh::send_message(handle, BM_SETCHECK, x as WPARAM, 0);
    }

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button.
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the check box in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the check box in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the check box in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the check box in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the check box label
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the check box label
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("BUTTON")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{BS_NOTIFY, WS_CHILD};

        BS_NOTIFY | WS_CHILD 
    }

    /// Change the checkbox background color.
    fn hook_background_color(&self, c: [u8; 3]) {
        use crate::bind_raw_event_handler;
        use winapi::um::winuser::{WM_CTLCOLORSTATIC};
        use winapi::shared::{basetsd::UINT_PTR, windef::{HWND}, minwindef::LRESULT};
        use winapi::um::wingdi::{CreateSolidBrush, RGB};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let parent_handle = ControlHandle::Hwnd(wh::get_window_parent(handle));
        let brush = unsafe { CreateSolidBrush(RGB(c[0], c[1], c[2])) };
        
        bind_raw_event_handler(&parent_handle, handle as UINT_PTR, move |_hwnd, msg, _w, l| {
            match msg {
                WM_CTLCOLORSTATIC => {
                    let child = l as HWND;
                    if child == handle {
                        return Some(brush as LRESULT);
                    }
                },
                _ => {}
            }

            None
        });
    }

}

pub struct CheckBoxBuilder<'a> {
    text: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    background_color: Option<[u8; 3]>,
    check_state: CheckBoxState,
    flags: Option<CheckBoxFlags>,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>
}

impl<'a> CheckBoxBuilder<'a> {

    pub fn flags(mut self, flags: CheckBoxFlags) -> CheckBoxBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn text(mut self, text: &'a str) -> CheckBoxBuilder<'a> {
        self.text = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> CheckBoxBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> CheckBoxBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn check_state(mut self, check: CheckBoxState) -> CheckBoxBuilder<'a> {
        self.check_state = check;
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> CheckBoxBuilder<'a> {
        self.background_color = color;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> CheckBoxBuilder<'a> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> CheckBoxBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut CheckBox) -> Result<(), SystemError> {
        let mut flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());
        if flags & BS_AUTO3STATE == 0 {
            flags |= BS_AUTOCHECKBOX;
        }

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(SystemError::ControlWithoutParent)
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .text(self.text)
            .parent(Some(parent))
            .build()?;

        if self.font.is_some() {
            out.set_font(self.font);
        }

        if self.background_color.is_some() {
            out.hook_background_color(self.background_color.unwrap());
        }

        out.set_check_state(self.check_state);

        Ok(())
    }

}
