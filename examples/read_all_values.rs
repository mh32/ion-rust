use ion_rs::raw_reader::StreamItem;
use ion_rs::result::IonResult;
use ion_rs::{IonDataSource, IonType, RawBinaryReader, RawReader};
use std::fs::File;
use std::process::exit;

fn main() -> IonResult<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).unwrap_or_else(|| {
        eprintln!("USAGE:\n\n    {} [Binary Ion file]\n", args.get(0).unwrap());
        eprintln!("No input file was specified.");
        exit(1);
    });

    let file = File::open(path)?;
    let buf_reader = std::io::BufReader::new(file);
    let mut cursor = RawBinaryReader::new(buf_reader);
    let number_of_values = read_all_values(&mut cursor)?;
    println!("Read {} values", number_of_values);
    Ok(())
}

// Visits each value in the stream recursively, reading each scalar into a native Rust type.
// Prints the total number of values read upon completion.
fn read_all_values<R: IonDataSource>(cursor: &mut RawBinaryReader<R>) -> IonResult<usize> {
    use IonType::*;
    use StreamItem::*;
    let mut count: usize = 0;
    loop {
        match cursor.next()? {
            Some(VersionMarker(_major, _minor)) => {}
            Some(StreamItem::Value(ion_type, is_null)) => {
                count += 1;
                if is_null {
                    continue;
                }
                match ion_type {
                    Struct | List | SExpression => cursor.step_in()?,
                    String => {
                        let _text = cursor.string_ref_map(|_s| ())?.unwrap();
                    }
                    Symbol => {
                        let _symbol_id = cursor.read_symbol()?.unwrap();
                    }
                    Integer => {
                        let _int = cursor.read_i64()?.unwrap();
                    }
                    Float => {
                        let _float = cursor.read_f64()?.unwrap();
                    }
                    Decimal => {
                        let _decimal = cursor.read_decimal()?.unwrap();
                    }
                    Timestamp => {
                        let _timestamp = cursor.read_timestamp()?.unwrap();
                    }
                    Boolean => {
                        let _boolean = cursor.read_bool()?.unwrap();
                    }
                    Blob => {
                        let _blob = cursor.blob_ref_map(|_b| ())?.unwrap();
                    }
                    Clob => {
                        let _clob = cursor.clob_ref_map(|_c| ())?.unwrap();
                    }
                    Null => {}
                }
            }
            None if cursor.depth() > 0 => {
                cursor.step_out()?;
            }
            _ => break,
        }
    }
    Ok(count)
}
