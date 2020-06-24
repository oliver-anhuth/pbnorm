use std::fs;
use std::io;
use regex::Regex;

fn pbnorm() -> io::Result<()> {
    let re = Regex::new(r"PB_Kontoauszug_KtoNr_\d{10}_(\d{2})-(\d{2})-(\d{4})_\d{6}.pdf$").unwrap();
    for entry in fs::read_dir(".")? {
        let file = entry?.path();
        if let Some(file) = file.to_str() {
            if let Some(captures) = re.captures(file) {
                let rename_to = format!(
                    "Postbank-{}-{}-{}.pdf",
                    captures.get(3).unwrap().as_str(),
                    captures.get(2).unwrap().as_str(),
                    captures.get(1).unwrap().as_str());
                let result = fs::rename(file, &rename_to);
                if result.is_err() {
                    println!("Could not rename {} to {}", &file, &rename_to);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let result = pbnorm();
    if result.is_err() {
        println!("Could not process files");
    }
}

#[cfg(test)]
mod tests {

}
