use std::fs::File;
use std::path::{Path, PathBuf};
use sms_backup_reader::messages::{Message, MessagesReader};
use sms_backup_reader::ExporterType;

fn export_messages(input_path: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(&input_path)?;
    let reader = MessagesReader::new(input_file)?;
    for message in reader {
        let contact = message.address.replace("+", "");
        let date = message.date.format("%Y-%m-%d").to_string();
        let year = message.date.format("%Y").to_string();
        let month = message.date.format("%m").to_string();
        let time = message.date.format("%H\t%M\t%S\t").to_string();
        let text = message.text.replace("\n", " ");
        let filename = format!("{}-{}.tsv", date, contact);
        let dirpath = output_dir.join(&contact).join(&year).join(&month).join(&date);
        std::fs::create_dir_all(&dirpath)?;
        let filepath = dirpath.join(&filename);
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filepath)?;
        writeln!(file, "{}\t{}\t{}", if message.type_ == 1 { "THEM" } else { "ME" }, time, text)?;
    }
    Ok(())
}
