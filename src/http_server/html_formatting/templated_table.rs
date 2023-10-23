pub fn templated_table(values: Vec<String>, columns: i32) -> Result<String, String>{
    let mut html_code = "<table>".to_string();
    let mut header = true;

    for row in 0..(values.len() / columns as usize){
        let result = row_builder((row*columns as usize), columns as usize, header, &values);
        match result {
            Ok(str) => {
                html_code.push_str(&str);
            },
            Err(err) => {
                return Err(err);
            }
        }
        header = false;
    }
    html_code.push_str("</table>");

    return Ok(html_code);
}

fn row_builder(start: usize, col_length: usize, header: bool, values: &Vec<String>) -> Result<String, String>{
    let mut html_code = "<tr>".to_string();

    let mut cell_html = "td";
    if header{
        cell_html = "th";
    }

    for column in start..(col_length+start)
    {
        let value = values.get(column);
        match value{
            Some(val) => {
                html_code.push_str(format!("<{}>{}</{}>", cell_html, val, cell_html).as_str())
            },
            None => {
                return Err("Failed to get value. Out of bounds?".to_owned());
            }
        }
    }
    html_code.push_str("</tr>");
    return Ok(html_code);
}