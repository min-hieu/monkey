use std::cmp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn print_box(quote: String, line_width: usize) -> String {
    let quote_bytes = quote.as_bytes();

    if quote.len() == 0 {
        return String::from("");
    }

    if quote.len() < line_width {
        let cage = String::from("-").repeat(quote.len() + 2);
        let mut out = vec![];
        out.push(format!("/{cage}\\"));
        out.push(format!("| {quote} |"));
        out.push(format!("\\{cage}/"));
        return out.join("\n");
    }

    let mut out = vec![];
    let cage = String::from("-").repeat(line_width + 2);
    out.push(format!("/{cage}\\"));

    let mut offset = 0;
    let mut i = 0;

    let mut end = 0;
    let mut start;
    while end < quote.len() {
        start = i * line_width - offset;
        end = cmp::min((i + 1) * line_width - offset, quote.len());
        let mut pad = String::from("");
        while end < quote.len()
            && end > 0
            && quote_bytes[end - 1] != b' '
            && quote_bytes[end] != b' '
        {
            offset += 1;
            end -= 1;
        }
        if quote_bytes[start] == b' ' {
            start += 1;
        }
        if end - start < line_width {
            let n_repeat = start + line_width - end;
            pad = String::from(" ").repeat(n_repeat);
        }
        let sub_quote = &quote[start..end];
        out.push(format!("| {sub_quote}{pad} |"));
        i += 1;
    }

    out.push(format!("\\{cage}/"));
    return out.join("\n");
}

#[wasm_bindgen]
pub fn print_monkey(offset: usize) -> String {
    let mut out = vec![];
    let monkey = vec![
        String::from("  \\             "),
        String::from("w  c(..)o   (    "),
        String::from(" \\__(-)    __)  "),
        String::from("     /\\   (     "),
        String::from("    /(_)___)     "),
        String::from("    w /|         "),
        String::from("     | \\        "),
        String::from("    m  m         "),
    ];
    for i in 0..monkey.len() {
        let str_offset = String::from(" ").repeat(offset);
        let fmt_monkey = str_offset + &monkey[i];
        out.push(format!("{fmt_monkey}"));
    }
    return out.join("\n");
}

#[wasm_bindgen]
pub fn monkey_talk(text: String, width: usize) -> String {
    let mut out = vec![];
    let monkey_offset = cmp::min(text.len(), width) / 2;
    out.push(print_box(String::from(text), width));
    out.push(print_monkey(monkey_offset));
    return out.join("\n");
}
