use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub(crate) fn get_2d_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    const CONTEXT_ID: &str = "2d";
    const ERROR_MESSAGE: &str = "unable to get 2d context";

    let context = canvas
        .get_context(CONTEXT_ID)
        .expect(ERROR_MESSAGE)
        .expect(ERROR_MESSAGE);

    // This is safe because get_context() always returns `CanvasRenderingContext2d`
    // when the context id '2d' is passed.
    unsafe { std::mem::transmute(context) }
}
