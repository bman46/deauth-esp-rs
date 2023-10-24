pub fn templated_link(text: impl AsRef<str>, url: impl AsRef<str>) -> String{
    format!("<a href=\"{}\">{}</a>", url.as_ref(), text.as_ref())
}