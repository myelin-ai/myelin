use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub(crate) fn get_2d_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    const CONTEXT_TYPE: &str = "2d";
    const ERROR_MESSAGE: &str = "unable to get 2d context";

    let context = canvas
        .get_context(CONTEXT_TYPE)
        .expect(ERROR_MESSAGE)
        .expect(ERROR_MESSAGE);

    unsafe { std::mem::transmute(context) }
}
