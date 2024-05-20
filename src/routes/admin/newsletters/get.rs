use actix_web::HttpResponse;

pub async fn newsletters_form() -> HttpResponse {
    HttpResponse::Ok().body(
        r#"
        <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Newsletters</title>
</head>
<body>
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

        <button type="submit">Publish</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>
        "#,
    )
}
