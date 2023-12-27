use neon::prelude::*;
use sha2::{Digest, Sha256};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("split_and_sha256", spit_sentence)?;

    Ok(())
}

fn spit_sentence(mut cx: FunctionContext) -> JsResult<JsArray> {
    let s = cx.argument::<JsString>(0)?.value(&mut cx) as String;

    let tokenized_vec: Vec<u64> = nomalize_input(&s)
        .split_terminator(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|word| {
            // let tokenized_word =
            Sha256::digest(word.as_bytes())
                .iter()
                .take(8)
                .copied()
                .collect::<Vec<_>>()
                .iter()
                .enumerate()
                .fold(0, |acc, (index, &byte)| {
                    acc + (byte as u64) * 256u64.pow(index as u32)
                })

            // let mut hasher = Blake2s256::new();
            // hasher.update(word);
            // hasher.finalize()[..]
        })
        .collect();

    //convert to js array
    let a = JsArray::new(&mut cx, tokenized_vec.len() as u32);
    for (i, s) in tokenized_vec.iter().enumerate() {
        let v = cx.number(*s as f64);
        let _ = a.set(&mut cx, i as u32, v);
    }
    Ok(a)
}

pub fn nomalize_input(input: &str) -> String {
    let input =
        input
            .trim()
            .chars()
            .map(|x| match x {
                'À' | 'Á' | 'Ả' | 'Ạ' | 'Ã' | 'Â' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' | 'Ä' | 'Å'
                | 'Æ' | 'Ă' | 'Ắ' | 'Ằ' | 'Ẵ' | 'Ẳ' | 'Ặ' => 'a',
                'Þ' => 'b',
                'Ç' | 'Č' => 'c',
                'Ď' | 'Ð' => 'd',
                'Ě' | 'È' | 'É' | 'Ẽ' | 'Ẻ' | 'Ẹ' | 'Ê' | 'Ế' | 'Ề' | 'Ễ' | 'Ể' | 'Ệ' | 'Ë' => {
                    'e'
                }
                'Ƒ' => 'f',
                'Ì' | 'Í' | 'Ĩ' | 'Ỉ' | 'Ị' | 'Î' | 'Ï' => 'i',
                'Ň' | 'Ñ' => 'n',
                'Ò' | 'Ó' | 'Õ' | 'Ỏ' | 'Ọ' | 'Ô' | 'Ố' | 'Ồ' | 'Ỗ' | 'Ổ' | 'Ộ' | 'Ơ' | 'Ớ'
                | 'Ờ' | 'Ỡ' | 'Ở' | 'Ợ' | 'Ö' | 'Ø' => 'o',
                'Ř' => 'r',
                'Š' => 's',
                'Ť' => 't',
                'Ů' | 'Ù' | 'Ú' | 'Ũ' | 'Ủ' | 'Ụ' | 'Ư' | 'Ứ' | 'Ừ' | 'Ữ' | 'Ử' | 'Ự' | 'Û'
                | 'Ü' => 'u',
                'Ý' | 'Ỳ' | 'Ỹ' | 'Ỷ' | 'Ỵ' => 'y',
                'Ž' => 'z',

                'à' | 'á' | 'ã' | 'ả' | 'ạ' | 'â' | 'ấ' | 'ầ' | 'ẫ' | 'ẩ' | 'ậ' | 'ă' | 'ắ'
                | 'ằ' | 'ẵ' | 'ẳ' | 'ặ' | 'ä' | 'å' | 'æ' => 'a',
                'þ' => 'b',
                'ç' | 'č' => 'c',
                'ď' | 'ð' | 'đ' => 'd',
                'ě' | 'è' | 'é' | 'ẽ' | 'ẻ' | 'ẹ' | 'ê' | 'ế' | 'ề' | 'ễ' | 'ể' | 'ệ' | 'ë' => {
                    'e'
                }
                'ƒ' => 'f',
                'ì' | 'í' | 'ĩ' | 'ỉ' | 'ị' | 'î' | 'ï' => 'i',
                'ñ' | 'ň' => 'n',
                'ò' | 'ó' | 'õ' | 'ỏ' | 'ọ' | 'ô' | 'ố' | 'ồ' | 'ỗ' | 'ổ' | 'ộ' | 'ơ' | 'ớ'
                | 'ờ' | 'ỡ' | 'ở' | 'ợ' | 'ö' | 'ø' => 'o',
                'ř' => 'r',
                'š' => 's',
                'ť' => 't',
                'ů' | 'ù' | 'ú' | 'ũ' | 'ủ' | 'ụ' | 'ư' | 'ứ' | 'ừ' | 'ữ' | 'ử' | 'ự' | 'û'
                | 'ü' => 'u',
                'ý' | 'ỳ' | 'ỹ' | 'ỷ' | 'ỵ' | 'ÿ' => 'y',
                'ž' => 'z',
                'A'..='Z' => x.to_ascii_lowercase(),
                'a'..='z' => x,
                '0'..='9' => x,
                _ => ' ',
            })
            // .filter(|c| c.is_ascii())
            .collect::<String>();

    input
}
