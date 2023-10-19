pub fn templated_html(title: impl AsRef<str> ,content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>{}</title>
        <style>
            table {{
            font-family: arial, sans-serif;
            border-collapse: collapse;
            width: 100%;
            }}

            td, th {{
            border: 1px solid #dddddd;
            text-align: left;
            padding: 8px;
            }}

            tr:nth-child(even) {{
            background-color: #dddddd;
            }}
        </style>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        title.as_ref(),
        content.as_ref()
    )
}