mod menus;

use std::str::FromStr;

use terminal_menu::{mut_menu, run};

use crate::subnautica::prelude::*;

pub fn run_shell() {
    let mut base = Base::new();

    loop {
        let main_menu = menus::main_menu(&mut base);

        run(&main_menu);

        let selected_section = {
            let m = mut_menu(&main_menu);

            m.selected_item_name().to_string()
        };

        if selected_section  == "exit" { break; }

        let mut mutable_menu = mut_menu(&main_menu);

        let subsection = mutable_menu.get_submenu(selected_section.as_str());

        match selected_section.as_str() {
            "add" => {
                let item = subsection.selection_value("Base Piece").replace(" ", "").to_lowercase();
                let quantity = subsection.numeric_value("Quantity") as usize;

                // Infallible since all menu items are valid
                base.add_item(Item::from_str(item.as_str()).unwrap(), quantity);
            },
            "remove" => {
                let item = subsection.selection_value("Base Piece").replace(" ", "").to_lowercase();
                let quantity = subsection.numeric_value("Quantity") as usize;
                println!("Item: {item:?}");
                println!("Quantity: {quantity:?}");

                // Infallible since all menu items are valid
                base.remove_item(Item::from_str(item.as_str()).unwrap(), quantity);
            },
            "depth" => {
                let new_depth = subsection.numeric_value("Set Depth:") as u32;
                base.depth = new_depth;
            }
            _ => (),
        };
    }
}
