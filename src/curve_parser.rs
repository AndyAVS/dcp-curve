/* Copyright © 2025 AndyAVS */

use quick_xml::Reader;
use quick_xml::events::Event;
use std::error::Error;

const TONE_CURVE_TAG: &[u8] = b"ToneCurve";
const ELEMENT_TAG: &[u8] = b"Element";

pub fn print_values(h_values: &Vec<f64>, v_values: &Vec<f64>) {
    for (i, (h, v)) in h_values.iter().zip(v_values.iter()).enumerate() {
        println!("Точка {}: h = {}, v = {}", i, h, v);
    }
}

pub fn parse_tone_curve_data(xml_content: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut h_values: Vec<f64> = Vec::new();
    let mut v_values: Vec<f64> = Vec::new();
    let mut buf: Vec<u8> = Vec::new();
    let mut is_tone_curve = false;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Empty(e) | Event::Start(e) => {
                if e.name().as_ref() == TONE_CURVE_TAG{
                    is_tone_curve = true;
                }

                if e.name().as_ref() == ELEMENT_TAG && is_tone_curve {
                    let mut option_h = None;
                    let mut option_v = None;

                    for attr in e.attributes() {
                        let attr = attr?;
                        let key = attr.key.as_ref();
                        let value = std::str::from_utf8(&attr.value)?;

                        if key == b"h" {
                            option_h = Some(value.parse::<f64>()?);
                        } else if key == b"v" {
                            option_v = Some(value.parse::<f64>()?);
                        }
                    }

                    if let (Some(h_val), Some(v_val)) = (option_h, option_v) {
                        h_values.push(h_val);
                        v_values.push(v_val);
                    }
                }
            },
            Event::End(e) => {
                if e.name().as_ref() == TONE_CURVE_TAG{
                    is_tone_curve = false;
                }
            },
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }

    Ok((h_values, v_values))
}
