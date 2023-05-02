use std::fs::File;
use std::path::{Path, PathBuf};
use regex::Regex;
use sms_backup_reader::messages::{Message, MessagesReader};
use sms_backup_reader::ExporterType;

fn export_messages(input_path: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(&input_path)?;
    let reader = MessagesReader::new(input_file)?;
    for message in reader {
        let contact = message.address.replace("+", "");
        let sanitized_contact = sanitize_filename(&contact);
        let date = message.date.format("%Y-%m-%d").to_string();
        let year = message.date.format("%Y").to_string();
        let month = message.date.format("%m").to_string();
        let time = message.date.format("%H\t%M\t%S\t").to_string();
        let text = message.text.replace("\n", " ");
        let filename = format!("{}-{}.tsv", date, sanitized_contact);
        let dirpath = output_dir.join(&sanitized_contact).join(&year).join(&month).join(&date);
        std::fs::create_dir_all(&dirpath)?;
        let filepath = dirpath.join(&filename);
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filepath)?;
        writeln!(file, "{}\t{}\t{}", if message.type_ == 1 { "THEM" } else { "US" }, time, text)?;
    }
    Ok(())
}

// Function to sanitize the contact name
fn sanitize_filename(filename: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
    re.replace_all(filename, "_").into_owned()
}
