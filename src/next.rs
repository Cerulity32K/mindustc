use crate::is;

pub fn next_char(i: &mut usize, v: &Vec<char>) -> Option<char> {
    *i = *i + 1;
    v.get(*i - 1).copied()
}

pub fn next_identifier(i: &mut usize, v: &Vec<char>) -> String {
    let mut out = String::new();
    while let Some(ch) = v.get(*i) {
        if is::is_identifier_char(*ch) {
            *i += 1;
            out.push(*ch)
        } else {
            break;
        }
    }
    out
}

pub fn next_number(i: &mut usize, v: &Vec<char>, negative: bool) -> f32 {
    let mut out = 0.0;
    'lloop: while let Some(ch) = v.get(*i) {
        if ('0'..='9').contains(ch) {
            out *= 10.0;
            out += (*ch as u8 - '0' as u8) as f32;
            *i += 1;
        } else if *ch == '.' {
            *i += 1;
            let mut decimal = 0.0;
            let mut div10 = 1;
            while let Some(ch) = v.get(*i) {
                if ('0'..='9').contains(ch) {
                    div10 *= 10;
                    decimal += (*ch as u8 - '0' as u8) as f32 / div10 as f32;
                    *i += 1;
                } else {
                    out += decimal;
                    break 'lloop;
                }
            }
            out += decimal;
        } else {
            break;
        }
    }
    if negative {-out} else {out}
}
