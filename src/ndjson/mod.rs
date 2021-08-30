#[cfg(test)]
mod tests;

use anyhow::anyhow;
use anyhow::Result;
use serde_json::Value;
use std::io;

pub(super) struct Opts {
    pub group: bool,
    pub count: bool,
}

pub(super) fn uniq<R, O>(mut reader: R, key: &str, fout: O, opts: Opts) -> Result<()>
where
    R: io::prelude::BufRead,
    O: io::Write,
{
    let mut writer = io::BufWriter::new(fout);
    let mut prev: Option<Value> = None;
    let mut lines: Vec<Value> = Vec::new();

    loop {
        let curt = read_line_with_parsing(&mut reader)?;

        if curt.is_none() {
            break;
        }

        let row = curt.as_ref().unwrap();

        let val = if let Some(v) = row.get(key) {
            v
        } else {
            return Err(anyhow!("Key '{}' does not exist: {}", key, row));
        };

        if let Some(prev_row) = prev {
            let prev_val = prev_row.get(key).unwrap();

            if val != prev_val {
                print_line(&mut writer, prev_val, &lines, opts.group, opts.count)?;
                lines.clear();
            }
        }

        lines.push(row.clone());
        prev = curt;
    }

    if prev.is_some() && lines.len() > 0 {
        print_line(
            &mut writer,
            prev.unwrap().get(key).unwrap(),
            &lines,
            opts.group,
            opts.count,
        )?;
    }

    Ok(())
}

fn read_line_with_parsing<T>(reader: &mut T) -> Result<Option<Value>>
where
    T: io::BufRead,
{
    let mut line = String::new();
    let n = reader.read_line(&mut line)?;

    if n == 0 {
        return Ok(None);
    }

    let r: serde_json::error::Result<Value> = serde_json::from_str(&line);

    match r {
        Err(e) => {
            let ctx = format!("Failed to parse JSON: {}", &line);
            Err(anyhow::Error::new(e).context(ctx))
        }
        Ok(v) => {
            if !v.is_object() {
                Err(anyhow!("JSON in row is not Object type: {}", v))
            } else {
                Ok(Some(v))
            }
        }
    }
}

fn print_line<T>(
    writer: &mut T,
    val: &Value,
    lines: &Vec<Value>,
    group: bool,
    count: bool,
) -> io::Result<()>
where
    T: io::Write,
{
    let row = if group {
        let csv = lines
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join(",");

        if count {
            format!("[{},{},{}]", val, lines.len(), csv)
        } else {
            format!("[{},{}]", val, csv)
        }
    } else if count {
        format!("[{},{}]", val, lines.len())
    } else {
        lines.get(0).unwrap().to_string()
    };

    writeln!(writer, "{}", row)
}
