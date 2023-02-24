mod chrome_window;

use std::{rc::Rc, sync::Mutex};

use self::chrome_window::{ChromeWebContext, ChromeWebView, ChromeWindow};
use headless_chrome::browser::*;
use headless_webview::{
    webview::{web_context::WebContext, WebViewAttributes},
    EngineWebview, WindowBuilder, types::WindowSize,
};

pub fn headless() -> WindowBuilder<ChromeWindow> {
    println!("Hello, world!");

    // user_data_dir: (),
    // disable_default_args: (),
    let launch_options = LaunchOptionsBuilder::default()
        .headless(true)
        .sandbox(false)
        .ignore_certificate_errors(true)
        .build()
        .unwrap();
    let browser = Browser::new(launch_options).unwrap();
    let tab = browser.new_tab().unwrap();
    let builder = WindowBuilder::new(tab);
    // TODO: builder.add_before_build_fn(headless_prebuild)
    builder
}
// TODO: fn headless_prebuild // should set the window size

impl EngineWebview for ChromeWebView {
    type Window = ChromeWindow;
    type WebContext = ChromeWebContext;

    fn new(
        window: Rc<Self::Window>,
        _webview: WebViewAttributes<Self::Window>,
        _web_context: Option<Rc<Mutex<WebContext<Self::WebContext>>>>,
    ) -> Result<ChromeWebView, headless_webview::Error>
    where
        Self: Sized,
    {
        todo!();
    }

    fn window(&self) -> &Self::Window {
        todo!()
    }

    fn evaluate_script(&self, js: &str) -> headless_webview::Result<()> {
        todo!()
    }

    fn resize(&self, new_size: WindowSize) -> headless_webview::Result<()> {
        todo!()
    }

    fn version(&self) -> headless_webview::Result<String> {
        todo!()
    }

    fn send_keyboard_input(&self, keyboard_input: headless_webview::types::KeyboardInput) {
        todo!()
    }

    fn send_mouse_position(&self, position: headless_webview::types::Vec2) {
        todo!()
    }

    fn send_mouse_event(&self, mouse_event: headless_webview::types::MouseEvent) {
        todo!()
    }

    fn get_texture(
        &mut self,
    ) -> headless_webview::Result<Option<headless_webview::types::Texture>> {
        todo!()
    }

    fn tick_once(&mut self) {
        todo!()
    }

    fn close(&mut self) {
        todo!()
    }

    fn load_html(&self, html: String) {
        todo!()
    }

    fn load_uri(&self, uri: String) {
        todo!()
    }

    fn reload(&self) {
        todo!()
    }

    fn set_is_visible(&mut self, is_visible: bool) {
        todo!()
    }
}

fn init_script(webview: &Tab, js: &str) {
    let page = webview.wait_for_element("html").unwrap();
    page.call_js_fn(
        // function to add a script tag to the page
        "function(script) {
            document.body.appendChild('<script>' + script + '</script>')
        );}",
        vec![js.into()],
        false,
    )
    .unwrap();
}
