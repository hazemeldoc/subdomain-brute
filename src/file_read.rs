use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
pub fn read_file_line_by_line(
    filepath: &str,
    vec: &mut Vec<String>,
    domain: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = match File::open(filepath) {
        Ok(mut f) => f,
        Err(err) => panic!("unable to read from file error:{}", err.to_string()),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut x = line.unwrap().to_string();
        x.push_str(".");
        x.push_str(domain);
        vec.push(x);
    }
    Ok(())
}
