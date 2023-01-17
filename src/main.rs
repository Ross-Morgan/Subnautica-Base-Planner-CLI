use subnautica_base_planner::shell::run_shell;

use colored::Colorize;


fn main() {
    println!("{}", "Launching Subnautica Base Planner...".bright_green());
    println!("Navigate menus with {}, {} or the {}", "wasd".purple(), "hjkl".purple(), "arrow keys".purple());
    run_shell();
    println!("{}", "Goodbye :)".bright_green());
}
