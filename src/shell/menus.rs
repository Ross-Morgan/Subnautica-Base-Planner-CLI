use colored::Colorize;
use crossterm::style::Color;
use terminal_menu::{TerminalMenu, TerminalMenuItem, back_button, button, label, menu, numeric, scroll, submenu};

use crate::subnautica::prelude::Base;

pub fn help_menu() -> Vec<TerminalMenuItem> {
    vec![
        label("------------------------------"),
        back_button("Back"),
        label("Select command you need help with"),
        label("------------------------------"),
    ]
}

pub fn add_menu() -> Vec<TerminalMenuItem> {
    vec![
        label("------------------------------"),
        back_button("Back"),
        label("Select item to add to the base"),
        label("------------------------------"),
        scroll("Base Piece", vec![
            "Bulkhead",
            "Foundation",
            "Glass Compartment",
            "Hatch",
            "Large Room",
            "Large Room Glass Dome",
            "Moonpool",
            "Multipurpose Room",
            "Multipurpose Room Glass Dome",
            "Observatory",
            "Reinforcement",
            "Scanner Room",
            "Titanium Compartment",
            "Vertical Connector",
            "Water Filtration Machine",
            "Window",
        ]).colorize(Color::Yellow),
        label("------------------------------"),
        numeric("Quantity", 1.0, Some(1.0), Some(1.0), None).colorize(Color::AnsiValue(202)),
        label( "------------------------------"),
        button(format!("{:^30}", "Ok".green())),
        label( "------------------------------"),
    ]
}

pub fn remove_menu() -> Vec<TerminalMenuItem> {
    vec![
        label("------------------------------"),
        back_button("Back"),
        label("Select item to remove from the base"),
        label("------------------------------"),
        scroll("Base Piece", vec![
            "Bulkhead",
            "Foundation",
            "Glass Compartment",
            "Hatch",
            "Large Room",
            "Large Room Glass Dome",
            "Moonpool",
            "Multipurpose Room",
            "Multipurpose Room Glass Dome",
            "Observatory",
            "Reinforcement",
            "Scanner Room",
            "Titanium Compartment",
            "Vertical Connector",
            "Water Filtration Machine",
            "Window",
        ]).colorize(Color::Yellow),
        label("------------------------------"),
        numeric("Quantity", 1.0, Some(1.0), Some(1.0), None),
        label( "------------------------------"),
        button(format!("{:^30}", "Ok".green())),
        label( "------------------------------"),
    ]
}


pub fn list_menu(base: &mut Base) -> Vec<TerminalMenuItem> {
    let mut v = base.items
        .iter()
        .filter_map(|(item, c)| {
            match *c {
                0 => None,
                _ => Some((item.clone(), *c)),
            }
        })
        .map(|(i, c)| {
            label(format!("{} | {c}", i.as_ref()))
        })
        .collect::<Vec<_>>();

    v.push(label("----------------"));
    v.push(back_button("Back"));
    v.push(label("----------------"));

    v
}

pub fn depth_menu(base: &mut Base) -> Vec<TerminalMenuItem> {
    vec![
        label(format!("Current Depth: {} metres", base.depth)),
        label(format!("Current Integrity: {:.2}", base.get_integrity())),
        numeric("Set Depth:", base.depth as f64, Some(1.0), Some(0.0), None),
        label( "----------------"),
        button(format!("{:^16}", "Ok".green())),
        label( "----------------"),
    ]
}

pub(crate) fn main_menu(base: &mut Base) -> TerminalMenu {
    let menu_args = vec![
        submenu("help", help_menu()),
        submenu("add", add_menu()),
        submenu("remove", remove_menu()),
        submenu("list", list_menu(base)),
        submenu("depth", depth_menu(base)),
        label("--------"),
        button("exit"),
    ];

    menu(menu_args)
}
