use crate::parser;
use crate::utils;
const TR_HEADER: &str = r#"#[macro_use]
extern crate tr;"#;

const HEADER: &str = r#"
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
use fltk::window::*;"#;

/// Generate the output Rust string/file
pub fn generate(ast: &[parser::Token]) -> String {
    let mut decls = String::new();
    let mut s = String::new();
    let mut ctor = String::new();
    let mut imp = String::new();
    let mut subs = vec![];
    let mut last_scope = None;
    let mut last_ast = parser::TokenType::Global;
    let mut gparent: Vec<String> = vec![];
    for elem in ast {
        use parser::TokenType::*;
        match &elem.typ {
            Decl => {
                let temp: String = elem.ident.clone();
                let temp = temp.strip_prefix("decl {").unwrap();
                let temp = if let Some(close) = temp.rfind('}') {
                    temp.split_at(close).0
                } else {
                    temp
                };
                decls += temp;
                decls += "\n";
            }
            Class => {
                s += "#[derive(Debug, Clone)]\n";
                s += "pub struct ";
                s += &elem.ident;
                s += " {\n";
                imp += "impl ";
                imp += &elem.ident;
                imp += " {\n";
            }
            Function => {
                imp += "    pub fn ";
                imp += &elem.ident;
                if !elem.ident.contains("-> Self") {
                    imp += " -> Self";
                }
                imp += " {\n";
                ctor += "\tSelf { ";
            }
            Member(t, props) => {
                let t = if props.contains(&"class".to_string()) {
                    &props[props.iter().position(|x| x == "class").unwrap() + 1]
                } else {
                    t
                };
                if t != "MenuItem" && t != "Submenu" && !elem.ident.contains("fl2rust_widget_") {
                    if props.contains(&"comment".to_string()) {
                        s += &format!(
                            "    // {}\n",
                            utils::unbracket(
                                &props[props.iter().position(|x| x == "comment").unwrap() + 1]
                            )
                        );
                    }

                    s += &format!("    pub {}: {},\n", &elem.ident, t);
                    ctor += &elem.ident;
                    ctor += ", ";
                }
                let xywh = props.iter().position(|x| x == "xywh");
                let label = props.iter().position(|x| x == "label");
                let typ = props.iter().position(|x| x == "type");
                let is_parent = matches!(
                    t.as_str(),
                    "Window"
                        | "Group"
                        | "Pack"
                        | "Tabs"
                        | "Scroll"
                        | "Table"
                        | "Tile"
                        | "Wizard"
                        | "MenuBar"
                        | "MenuButton"
                        | "Choice"
                );
                if !is_parent {
                    if t != "MenuItem" && t != "Submenu" {
                        if let Some(xywh) = xywh {
                            imp += &format!(
                                "\tlet mut {} = {}::new({}, {}\n",
                                &elem.ident,
                                &t,
                                utils::unbracket(&props[xywh + 1].replace(" ", ", ")),
                                if let Some(l) = label {
                                    if unsafe { crate::parser::PROGRAM.i18n } {
                                        format!(
                                            "None).with_label(&tr!(\"{}\"));\n",
                                            utils::unbracket(&props[l + 1])
                                        )
                                    } else {
                                        format!("\"{}\");", utils::unbracket(&props[l + 1]))
                                    }
                                } else {
                                    "None);".to_string()
                                }
                            );
                        } else {
                            imp += &format!(
                                "\tlet mut {} = {}::default(){};\n",
                                &elem.ident,
                                &t,
                                if let Some(l) = label {
                                    if unsafe { crate::parser::PROGRAM.i18n } {
                                        format!(
                                            ".with_label(&tr!(\"{}\"));",
                                            utils::unbracket(&props[l + 1])
                                        )
                                    } else {
                                        format!(
                                            ".with_label(\"{}\");",
                                            utils::unbracket(&props[l + 1])
                                        )
                                    }
                                } else {
                                    String::new()
                                }
                            );
                        }
                    }
                } else if let Some(xywh) = xywh {
                    imp += &format!(
                        "\tlet mut {0} = {1}::new({2}, {3}\n\t{0}.end();\n",
                        &elem.ident,
                        &t,
                        utils::unbracket(&props[xywh + 1].replace(" ", ", ")),
                        if let Some(l) = label {
                            if unsafe { crate::parser::PROGRAM.i18n } {
                                format!(
                                    "None).with_label(&tr!(\"{}\"));",
                                    utils::unbracket(&props[l + 1])
                                )
                            } else {
                                format!("\"{}\");", utils::unbracket(&props[l + 1]))
                            }
                        } else {
                            "None);".to_string()
                        }
                    );
                } else {
                    imp += &format!(
                        "\tlet mut {0} = {1}::default(){2};\n\t{0}.end();\n",
                        &elem.ident,
                        &t,
                        if let Some(l) = label {
                            if unsafe { crate::parser::PROGRAM.i18n } {
                                format!(
                                    ".with_label(tr!(\"{}\"));",
                                    utils::unbracket(&props[l + 1])
                                )
                            } else {
                                format!(".with_label(\"{}\");", utils::unbracket(&props[l + 1]))
                            }
                        } else {
                            String::new()
                        }
                    );
                }
                for i in 0..props.len() {
                    match props[i].as_str() {
                        "visible" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!("\t{}.show();\n", &elem.ident,);
                                }
                            } else {
                                imp += &format!("\t{}.show();\n", &elem.ident,);
                            }
                        }
                        "color" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_color(Color::by_index({}));\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_color(Color::by_index({}));\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "selection_color" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_selection_color(Color::by_index({}));\n",
                                        &elem.ident,
                                        utils::global_to_pascal(utils::unbracket(&props[i + 1]))
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_selection_color(Color::by_index({}));\n",
                                    &elem.ident,
                                    utils::global_to_pascal(utils::unbracket(&props[i + 1]))
                                );
                            }
                        }
                        "labelsize" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_label_size({});\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_label_size({});\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "textsize" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_text_size({});\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_text_size({});\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "labeltype" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    let temp =
                                        utils::global_to_pascal(utils::unbracket(&props[i + 1]));
                                    let temp = if temp == "No" { "None" } else { temp.as_str() };
                                    imp += &format!(
                                        "\t{}.set_label_type(LabelType::{});\n",
                                        &elem.ident, temp,
                                    );
                                }
                            } else {
                                let temp = utils::global_to_pascal(utils::unbracket(&props[i + 1]));
                                let temp = if temp == "No" { "None" } else { temp.as_str() };
                                imp += &format!(
                                    "\t{}.set_label_type(LabelType::{});\n",
                                    &elem.ident, temp,
                                );
                            }
                        }
                        "labelcolor" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_label_color(Color::by_index({}));\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_label_color(Color::by_index({}));\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "labelfont" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_label_font(Font::by_index({}));\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_label_font(Font::by_index({}));\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "textfont" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_text_font(Font::by_index({}));\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_text_font(Font::by_index({}));\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "box" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    let temp =
                                        utils::global_to_pascal(utils::unbracket(&props[i + 1]));
                                    let temp = match temp.as_str() {
                                        "OflatBox" => "OFlatBox",
                                        "OshadowBox" => "OShadowBox",
                                        "RflatBox" => "RFlatBox",
                                        "RshadowBox" => "RShadowBox",
                                        _ => temp.as_str(),
                                    };
                                    imp += &format!(
                                        "\t{}.set_frame(FrameType::{});\n",
                                        &elem.ident, temp,
                                    );
                                }
                            } else {
                                let temp = utils::global_to_pascal(utils::unbracket(&props[i + 1]));
                                let temp = match temp.as_str() {
                                    "OflatBox" => "OFlatFrame",
                                    "OshadowBox" => "OShadowBox",
                                    "RflatBox" => "RFlatBox",
                                    "RshadowBox" => "RShadowBox",
                                    _ => temp.as_str(),
                                };
                                imp += &format!(
                                    "\t{}.set_frame(FrameType::{});\n",
                                    &elem.ident, temp,
                                );
                            }
                        }
                        "down_box" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    let temp =
                                        utils::global_to_pascal(utils::unbracket(&props[i + 1]));
                                    let temp = match temp.as_str() {
                                        "OflatBox" => "OFlatBox",
                                        "OshadowBox" => "OShadowBox",
                                        "RflatBox" => "RFlatBox",
                                        "RshadowBox" => "RShadowBox",
                                        _ => temp.as_str(),
                                    };
                                    imp += &format!(
                                        "\t{}.set_down_frame(FrameType::{});\n",
                                        &elem.ident, temp
                                    );
                                }
                            } else {
                                let temp = utils::global_to_pascal(utils::unbracket(&props[i + 1]));
                                let temp = match temp.as_str() {
                                    "OflatBox" => "OFlatBox",
                                    "OshadowBox" => "OShadowBox",
                                    "RflatBox" => "RFlatBox",
                                    "RshadowBox" => "RShadowBox",
                                    _ => temp.as_str(),
                                };
                                imp += &format!(
                                    "\t{}.set_down_frame(FrameType::{});\n",
                                    &elem.ident, temp
                                );
                            }
                        }
                        "when" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_trigger(unsafe {{std::mem::transmute({})}});\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_trigger(unsafe {{std::mem::transmute({})}});\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "tooltip" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_tooltip({});\n",
                                        &elem.ident,
                                        if unsafe { crate::parser::PROGRAM.i18n } {
                                            format!("&tr!(\"{}\")", utils::unbracket(&props[i + 1]))
                                        } else {
                                            format!("\"{}\"", utils::unbracket(&props[i + 1]))
                                        }
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_tooltip({});\n",
                                    &elem.ident,
                                    if unsafe { crate::parser::PROGRAM.i18n } {
                                        format!("&tr!(\"{}\")", utils::unbracket(&props[i + 1]))
                                    } else {
                                        format!("\"{}\"", utils::unbracket(&props[i + 1]))
                                    }
                                );
                            }
                        }
                        "maximum" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_maximum({} as _);\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_maximum({} as _);\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "minimum" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_minimum({} as _);\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_minimum({} as _);\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "step" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_step({} as _, 1);\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_step({} as _, 1);\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "value" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    let val = if t.contains("Button") {
                                        let b = utils::unbracket(&props[i + 1])
                                            .parse::<i32>()
                                            .expect("Buttons should have integral values");
                                        if b != 0 {
                                            "true".to_string()
                                        } else {
                                            "false".to_string()
                                        }
                                    } else if (t.contains("Input") || t.contains("Output"))
                                        && !t.contains("Value")
                                    {
                                        if unsafe { crate::parser::PROGRAM.i18n } {
                                            format!("&tr!(\"{}\")", utils::unbracket(&props[i + 1]))
                                        } else {
                                            format!("\"{}\"", utils::unbracket(&props[i + 1]))
                                        }
                                    } else {
                                        format!("{} as _", utils::unbracket(&props[i + 1]))
                                    };
                                    imp += &format!("\t{}.set_value({});\n", &elem.ident, val);
                                }
                            } else {
                                let val = if t.contains("Button") {
                                    let b = utils::unbracket(&props[i + 1])
                                        .parse::<i32>()
                                        .expect("Buttons should have integral values");
                                    if b != 0 {
                                        "true".to_string()
                                    } else {
                                        "false".to_string()
                                    }
                                } else if (t.contains("Input") || t.contains("Output"))
                                    && !t.contains("Value")
                                {
                                    if unsafe { crate::parser::PROGRAM.i18n } {
                                        format!("&tr!(\"{}\")", utils::unbracket(&props[i + 1]))
                                    } else {
                                        format!("\"{}\"", utils::unbracket(&props[i + 1]))
                                    }
                                } else {
                                    format!("{} as _", utils::unbracket(&props[i + 1]))
                                };
                                imp += &format!("\t{}.set_value({});\n", &elem.ident, val);
                            }
                        }
                        "type" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label"
                                    && props[i + 1] != "Double"
                                    && t != "MenuItem"
                                    && t != "Submenu"
                                {
                                    if t != "Output" {
                                        imp += &format!(
                                            "\t{}.set_type({}Type::{});\n",
                                            &elem.ident,
                                            utils::fix_type(t),
                                            utils::global_to_pascal(utils::unbracket(
                                                &props[i + 1]
                                            ))
                                        );
                                    } else {
                                        imp += &format!(
                                            "\t{}.set_type(InputType::from_i32(12));\n",
                                            &elem.ident,
                                        );
                                    }
                                }
                            } else if props[i + 1] != "Double" && t != "MenuItem" && t != "Submenu"
                            {
                                if t != "Output" {
                                    imp += &format!(
                                        "\t{}.set_type({}Type::{});\n",
                                        &elem.ident,
                                        utils::fix_type(t),
                                        utils::global_to_pascal(utils::unbracket(&props[i + 1]))
                                    );
                                } else {
                                    imp += &format!(
                                        "\t{}.set_type(InputType::from_i32(12));\n",
                                        &elem.ident,
                                    );
                                }
                            }
                        }
                        "align" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.set_align(unsafe {{std::mem::transmute({})}});\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.set_align(unsafe {{std::mem::transmute({})}});\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "shortcut" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" && t != "MenuItem" && t != "Submenu" {
                                    imp += &format!(
                                        "\t{}.set_shortcut(unsafe {{std::mem::transmute({})}});\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1])
                                    );
                                }
                            } else if t != "MenuItem" && t != "Submenu" {
                                imp += &format!(
                                    "\t{}.set_shortcut(unsafe {{std::mem::transmute({})}});\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1])
                                );
                            }
                        }
                        "image" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                    "\t{0}.set_image(Some(SharedImage::load(\"{1}\").expect(\"Could not find image: {1}\")));\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1]),
                                );
                                }
                            } else {
                                imp += &format!(
                                    "\t{0}.set_image(Some(SharedImage::load(\"{1}\").expect(\"Could not find image: {1}\")));\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1]),
                                );
                            }
                        }
                        "hide" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!("\t{}.hide();\n", &elem.ident,);
                                }
                            } else {
                                imp += &format!("\t{}.hide();\n", &elem.ident,);
                            }
                        }
                        "modal" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!("\t{}.make_modal(true);\n", &elem.ident,);
                                }
                            } else {
                                imp += &format!("\t{}.make_modal(true);\n", &elem.ident,);
                            }
                        }
                        "resizable" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" && is_parent {
                                    imp += &format!("\t{}.make_resizable(true);\n", &elem.ident,);
                                }
                            } else if is_parent {
                                imp += &format!("\t{}.make_resizable(true);\n", &elem.ident,);
                            }
                        }
                        "size_range" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!(
                                        "\t{}.size_range({});\n",
                                        &elem.ident,
                                        utils::unbracket(&props[i + 1].replace(" ", ", "))
                                    );
                                }
                            } else {
                                imp += &format!(
                                    "\t{}.size_range({});\n",
                                    &elem.ident,
                                    utils::unbracket(&props[i + 1].replace(" ", ", "))
                                );
                            }
                        }
                        "callback" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" && t != "MenuItem" && t != "Submenu" {
                                    let cb = utils::unbracket(&props[i + 1]);
                                    if cb.starts_with('{')
                                        || cb.starts_with("move")
                                        || cb.starts_with('|')
                                    {
                                        imp +=
                                            &format!("\t{}.set_callback({});\n", &elem.ident, cb);
                                    } else {
                                        imp += &format!(
                                            "\t{}.set_callback(move |{}| {{ \n\t    {} \n\t}});\n",
                                            &elem.ident, &elem.ident, cb
                                        );
                                    };
                                }
                            } else if t != "MenuItem" && t != "Submenu" {
                                let cb = utils::unbracket(&props[i + 1]);
                                if cb.starts_with('{')
                                    || cb.starts_with("move")
                                    || cb.starts_with('|')
                                {
                                    imp += &format!("\t{}.set_callback({});\n", &elem.ident, cb);
                                } else {
                                    imp += &format!(
                                        "\t{}.set_callback(move |{}| {{ \n\t    {} \n\t}});\n",
                                        &elem.ident, &elem.ident, cb
                                    );
                                };
                            }
                        }
                        "code0" | "code1" | "code2" | "code3" => {
                            if let Some(p) = props.get(i.wrapping_sub(1)) {
                                if p != "label" {
                                    imp += &format!("\t{}\n", utils::unbracket(&props[i + 1]));
                                }
                            } else {
                                imp += &format!("\t{}\n", utils::unbracket(&props[i + 1]));
                            }
                        }
                        _ => (),
                    }
                }
                if !gparent.is_empty() && !gparent.last().unwrap().contains("Function") {
                    if t != "MenuItem" && t != "Submenu" {
                        let parent = gparent.last().unwrap().clone();
                        let parent: Vec<&str> = parent.split_whitespace().collect();
                        let parent = parent[1];
                        imp += &format!("\t{}.add(&{});\n", parent, &elem.ident);
                        if props.contains(&"resizable".to_string()) {
                            imp += &format!("\t{}.resizable(&{});\n", parent, &elem.ident);
                        }
                        if props.contains(&"hotspot".to_string()) {
                            imp += &format!("\t{}.hotspot(&{});\n", parent, &elem.ident);
                        }
                    } else if t == "MenuItem" {
                        let mut menu_parent = String::new();
                        for p in gparent.iter().rev() {
                            if !p.contains("Submenu") {
                                menu_parent = p.clone();
                                break;
                            }
                        }
                        let parent: Vec<&str> = menu_parent.split_whitespace().collect();
                        let parent = parent[1];
                        let shortcut = if props.contains(&"shortcut".to_string()) {
                            Some(
                                props[props.iter().position(|r| r == "shortcut").unwrap() + 1]
                                    .clone(),
                            )
                        } else {
                            None
                        };
                        let cb = if props.contains(&"callback".to_string()) {
                            Some(
                                props[props.iter().position(|r| r == "callback").unwrap() + 1]
                                    .clone(),
                            )
                        } else {
                            None
                        };
                        imp += &format!(
                            "\t{}.add({}, {}, MenuFlag::{}, {});\n",
                            parent,
                            if let Some(l) = label {
                                if unsafe { crate::parser::PROGRAM.i18n } {
                                    format!(
                                        "&tr!(\"{}{}\")",
                                        utils::vec2menu(&subs),
                                        utils::unbracket(&props[l + 1])
                                    )
                                } else {
                                    format!(
                                        "\"{}{}\"",
                                        utils::vec2menu(&subs),
                                        utils::unbracket(&props[l + 1])
                                    )
                                }
                            } else {
                                "\"\"".to_string()
                            },
                            if let Some(shortcut) = shortcut {
                                format!("unsafe {{std::mem::transmute({})}}", shortcut)
                            } else {
                                "Shortcut::None".to_string()
                            },
                            if let Some(ty) = typ {
                                &props[ty + 1]
                            } else {
                                "Normal"
                            },
                            if let Some(cb) = cb {
                                let cb = utils::unbracket(&cb);
                                if cb.starts_with('{')
                                    || cb.starts_with("move")
                                    || cb.starts_with('|')
                                {
                                    cb.to_string()
                                } else {
                                    format!("move |{}| {{ \n\t\t{} \n\t}}", &parent, cb)
                                }
                            } else {
                                "|_| {}".to_string()
                            }
                        );
                    } else if t == "Submenu" {
                        subs.push(&elem.ident);
                    } else {
                        //
                    }
                }
            }
            Scope(op, p) => {
                if !*op {
                    if let Some(p) = p.last() {
                        if p.contains("Function") {
                            ctor += "}";
                            imp += &ctor;
                            imp += "\n    }\n";
                            ctor.clear();
                        }
                        if let parser::TokenType::Scope(false, _) = last_ast {
                            if let Some(parser::TokenType::Scope(false, last_parent)) = last_scope {
                                if let Some(l) = last_parent.last() {
                                    if l.contains("Submenu") || l.contains("Fl_") {
                                        subs.pop();
                                        gparent.pop();
                                    }
                                }
                            }
                        }
                    } else {
                        imp += "}\n\n";
                        s += "}\n\n";
                    }
                    last_scope = Some(parser::TokenType::Scope(false, p.clone()));
                } else {
                    gparent = p.clone();
                }
            }
            _ => (),
        }
        last_ast = elem.typ.clone();
    }
    if unsafe { crate::parser::PROGRAM.i18n } {
        return format!("{}\n{}\n\n{}\n\n{}\n{}\n", TR_HEADER, HEADER, decls, s, imp);
    }
    format!("{}\n\n{}\n\n{}\n{}\n", HEADER, decls, s, imp)
}
