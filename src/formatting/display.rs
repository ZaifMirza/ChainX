// Display traits and common display logic

use crate::config::constants::BOX_WIDTH;
use crate::utils::center_text;
use crate::utils::text::truncate_and_pad;

#[allow(dead_code)]
pub trait Displayable {
    fn display(&self) -> String;
}

#[allow(dead_code)]
pub fn print_box_header(title: &str) {
    println!("╔{}╗", "═".repeat(BOX_WIDTH - 2));
    println!("║{}║", center_text(title, BOX_WIDTH - 2));
    println!("╚{}╝", "═".repeat(BOX_WIDTH - 2));
}

#[allow(dead_code)]
pub fn print_box_line(label: &str, value: &str) {
    let label_padded = format!("{:18}", label);
    let value_truncated = truncate_and_pad(value, BOX_WIDTH - 25);
    println!("║  {} │ {}  ║", label_padded, value_truncated);
}

#[allow(dead_code)]
pub fn print_box_separator() {
    println!("╠{}╣", "═".repeat(BOX_WIDTH - 2));
}

#[allow(dead_code)]
pub fn print_box_footer() {
    println!("╚{}╝", "═".repeat(BOX_WIDTH - 2));
}

pub fn print_header(title: &str) {
    println!("\n");
    print_box_header(title);
    println!("║          Powered by Public RPC Nodes                     ║");
    print_box_footer();
    println!();
}
