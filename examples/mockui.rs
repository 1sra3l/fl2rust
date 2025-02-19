
// Automatically generated by fl2rust

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::image::*;
use fltk::input::*;
use fltk::menu::*;
use fltk::misc::*;
use fltk::output::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::text::*;
use fltk::tree::*;
use fltk::valuator::*;
use fltk::widget::*;
use fltk::window::*;

#[derive(Debug, Clone)]
pub struct UserInterface {
    pub win: Window,
    pub tabs: Tabs,
    pub page_1: Scroll,
    pub page_2: Scroll,
}


impl UserInterface {
    pub fn make_window() -> Self {
	let mut win = Window::new(392, 132, 240, 400, "Full Tick");
	win.end();
	win.set_color(Color::by_index(50));
	win.make_resizable(true);
	win.show();
	let mut tabs = Tabs::new(0, 25, 240, 375, "");
	tabs.end();
	tabs.set_frame(FrameType::FlatBox);
	tabs.set_color(Color::by_index(40));
	tabs.set_selection_color(Color::by_index(48));
	win.add(&tabs);
	let mut page_1 = Scroll::new(0, 50, 240, 350, "Page 1");
	page_1.end();
	page_1.set_frame(FrameType::FlatBox);
	page_1.set_color(Color::by_index(48));
	page_1.set_selection_color(Color::by_index(40));
	page_1.hide();
	tabs.add(&page_1);
	let mut page_2 = Scroll::new(0, 55, 240, 345, "Page 2");
	page_2.end();
	page_2.set_frame(FrameType::FlatBox);
	page_2.set_color(Color::by_index(48));
	page_2.set_selection_color(Color::by_index(40));
	tabs.add(&page_2);
	let mut fl2rust_widget_0 = Frame::new(0, 0, 70, 25, "@FLTK");
	win.add(&fl2rust_widget_0);
	let mut fl2rust_widget_1 = Frame::new(70, 0, 115, 25, "moCKontact");
	fl2rust_widget_1.set_label_font(Font::by_index(1));
	win.add(&fl2rust_widget_1);
	Self { win, tabs, page_1, page_2, }
    }
}



