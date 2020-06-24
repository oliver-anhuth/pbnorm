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
    use std::fs::{self, File};
    use std::path::Path;
    use super::pbnorm;

    #[test]
    fn test_pbnorm() {
        for file in [
            "PB_Kontoauszug_KtoNr_1234567890_06-01-2020_043030.pdf",
            "PB_Kontoauszug_KtoNr_1111111111_06-02-2020_043030.pdf",
            "PB_Kontoauszug_KtoNr_2222222222_06-03-2020_043030.pdf",
            "PB_Kontoauszug_KtoNr_3333333333_06-04-2020_043030.pdf",
            "PB_Kontoauszug_KtoNr_4444444444_06-05-2020_043030.pdf",
        ].iter() {
            File::create(file).expect("Could not create test file");
        }
        pbnorm().expect("Errors during test");
        for result_file in [
            "Postbank-2020-01-06.pdf",
            "Postbank-2020-02-06.pdf",
            "Postbank-2020-03-06.pdf",
            "Postbank-2020-04-06.pdf",
            "Postbank-2020-05-06.pdf",
        ].iter() {
            let result_file_path = Path::new(&result_file);
            assert!(result_file_path.is_file(), "Expected result file {} not found", result_file);
            fs::remove_file(result_file_path).ok();
        }
    }
}
