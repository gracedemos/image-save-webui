mod app;
mod banner;
mod utils;
mod find_image;
mod add_image;

use leptos::*;
use app::App;

fn main() {
    mount_to_body(|| view! {<App/>});
}
