use rust_xlsxwriter::{Format, Workbook};

/// Write a spreadsheet to `path` (.xlsx).
pub fn write_xlsx(path: &str, headers: &[String], rows: &[Vec<String>]) -> Result<(), String> {
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    let bold = Format::new().set_bold();
    for (c, h) in headers.iter().enumerate() {
        ws.write_string(0, c as u16, h).map_err(|e| e.to_string())?;
        ws.set_column_width(c as u16, 18).map_err(|e| e.to_string())?;
    }
    ws.set_row_format(0, &bold).map_err(|e| e.to_string())?;

    for (r, row) in rows.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            // Try numeric for cleaner Excel cells, otherwise string.
            if let Ok(n) = cell.parse::<f64>() {
                ws.write_number((r as u32 + 1) as u32, c as u16, n)
                    .map_err(|e| e.to_string())?;
            } else {
                ws.write_string((r as u32 + 1) as u32, c as u16, cell)
                    .map_err(|e| e.to_string())?;
            }
        }
    }

    wb.save(path).map_err(|e| e.to_string())?;
    Ok(())
}

/// Write a CSV file to `path` with proper quoting.
pub fn write_csv(path: &str, headers: &[String], rows: &[Vec<String>]) -> Result<(), String> {
    let mut out = String::new();
    out.push_str(&csv_line(headers));
    for row in rows {
        out.push_str(&csv_line(row));
    }
    std::fs::write(path, out).map_err(|e| e.to_string())?;
    Ok(())
}

fn csv_line(fields: &[String]) -> String {
    let mut line = String::new();
    for (i, f) in fields.iter().enumerate() {
        if i > 0 {
            line.push(',');
        }
        if f.contains(',') || f.contains('"') || f.contains('\n') || f.contains('\r') {
            line.push('"');
            line.push_str(&f.replace('"', "\"\""));
            line.push('"');
        } else {
            line.push_str(f);
        }
    }
    line.push('\n');
    line
}
