use poem_openapi::payload::PlainText;


pub fn handler() -> PlainText<&'static str> {
    PlainText("OK")
}
