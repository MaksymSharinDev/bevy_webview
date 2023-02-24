use std::{
    rc::Rc,
    sync::{Mutex, RwLock, Arc},
};

use headless_chrome::Tab;
use headless_webview::{
    types::WindowSize,
    webview::{
        web_context::{WebContext, WebContextData, WebContextImpl},
        WebViewAttributes,
    },
    window::WindowAttributes,
    EngineWebview, Error, HeadlessWindow,
};

pub struct ChromeWindow {
    inner_size: RwLock<WindowSize>,
}
pub struct ChromeWebView {
    window: Rc<ChromeWindow>,
}

pub struct ChromeWebContext {}

impl HeadlessWindow for ChromeWindow {
    type NativeWindow = Arc<Tab>;
    type Webview = ChromeWebView;

    fn new(_native_window: Self::NativeWindow, attributes: WindowAttributes) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(ChromeWindow {
            inner_size: RwLock::new(attributes.get_inner_size()),
        })
    }

    fn id(&self) -> headless_webview::window::WindowId {
        todo!()
    }

    fn inner_size(&self) -> WindowSize {
        todo!()
    }

    fn width(&self) -> u32 {
        self.inner_size().width
    }

    fn height(&self) -> u32 {
        self.inner_size().height
    }

    fn resize(&self, new_size: WindowSize) -> headless_webview::Result<()> {
        todo!()
    }
}


impl WebContextImpl for ChromeWebContext {
    fn new(_data: &WebContextData) -> Self {
        Self {}
    }
    fn set_allows_automation(&mut self, _flag: bool) {}
}
