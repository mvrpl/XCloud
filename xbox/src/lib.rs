use web_view::*;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn xbox() {
    web_view::builder()
        .title("XCloud")
        .content(Content::Url("https://xbox.com/pt-BR/play"))
        .size(3840, 2160)
        .resizable(true)
        .debug(false)
        .user_data("")
        .invoke_handler(|webview, arg| {
            match arg {
                "enter" => webview.set_fullscreen(true),
                "exit" => webview.set_fullscreen(false),
                _ => (),
            }
            Ok(())
        })
        .run()
        .unwrap();
}