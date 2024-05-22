use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn newsletters_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    let idempotency_key = uuid::Uuid::new_v4().to_string();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
        <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Newsletters</title>
</head>
<body>
{msg_html}
    <form action="/admin/newsletters" method="post">
        <label>
            Title
            <input
                type="text"
                placeholder="New Title"
                name="title"
            >
        </label>
        <br>
        <label for="text_content">Plain Text</label>
        <textarea id="text_content" name="text_content">
            Newsletter body
        </textarea>
        <br>
        <label for="html_content">HTML</label>
        <textarea id="html_content" name="html_content">Newsletter body</textarea>

        <input hidden type="test" name="idempotency_key" value="{idempotency_key}">
        <button type="submit">Publish</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>
        "#,
        ))
}
