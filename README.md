# About 
snar3 provides a convenient way refactor SMS message export/backup from an `XML` file to `TSV` files in a structured and organized format that makes it easy to search, sort, and analyze the data.
Specifically, it groups the messages by contact, year, month, and day, and exports them to TSV files in nested directories based on this grouping. Each TSV file contains the messages for a specific day and has columns for the `sender`, `timestamp`, and `message text`.

## Requirements

- Rust (Programming language)
- `sms-backup-reader` crate
- `chrono` crate
- `regex` crate

You can find the specific versions of the crates in the `Cargo.toml` file.

## Installation

1. Make sure you have Rust installed on your system. You can download it from [the official Rust website](https://www.rust-lang.org/tools/install)

2. Clone this repository or download the source code.

3. In a terminal, navigate to the project directory.

4. Run the following command to build the project and install the dependencies:
`cargo build`


## Usage

To run the program, use the following command:
```
cargo run -- <input_file_path> <output_dir_path>
```

Replace `<input_file_path>` with the path to your SMS backup XML file, and `<output_dir_path>` with the path to the directory where you want to save the exported TSV files.

For example:
```
cargo run -- ~/sms-backup.xml ~/Desktop/sms-export
```

The program will parse the XML file and export the SMS messages to the specified output directory in the TSV format.

## Example
The program can handle both incoming and outgoing messages and indicates which is which by adding `US` to outgoing messages and `THEM` to incoming messages in the sender column.

### Contents of one of resulting TSV files:
```
THEM    08  23  04      Hello, how are you?
US     08  24  31      I'm good, thanks. How about you?
THEM    08  25  17      Doing well, thanks.
```

### File naming and organizational method
In this example, the root directory contains the `snar3` directory where the program's source code resides, the sms-backup.xml file, and the `exports` directory where the exported files are saved.
```
.
├── sms-exporter
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── sms-backup.xml
└── exports
    ├── contact1
    │   ├── 2022
    │   │   └── 08
    │   │       └── 01
    │   │           └── 2022-08-01-contact1.tsv
    │   ├── 2022
    │   │   └── 08
    │   │       └── 02
    │   │           └── 2022-08-02-contact1.tsv
    │   └── 2022
    │       └── 09
    │           └── 01
    │               └── 2022-09-01-contact1.tsv
    ├── contact2
    │   ├── 2022
    │   │   └── 08
    │   │       └── 01
    │   │           └── 2022-08-01-contact2.tsv
    │   └── 2022
    │       └── 08
    │           └── 02
    │               └── 2022-08-02-contact2.tsv
    └── contact3
        ├── 2022
        │   └── 08
        │       └── 01
        │           └── 2022-08-01-contact3.tsv
        └── 2022
            └── 09
                └── 01
                    └── 2022-09-01-contact3.tsv
```

Inside the exports directory, there are subdirectories for each `contact`, named contact1, contact2, contact3, and so on. Each contact directory contains subdirectories for each year, such as 2022, and further subdirectories for each month, such as 08 (August) or 09 (September).

Within the month directories, there are subdirectories for each day, such as 01 or 02. Finally, within the day directories, TSV files are created with the naming convention `<date>-<contact>.tsv`, where <date> represents the date of the messages and <contact> represents the contact's name or identifier.

For example, the file '2022-08-01-contact1.tsv' contains the SMS messages for contact1 on August 1st, 2022. Similarly, the file '2022-08-02-contact2.tsv' contains the SMS messages for contact2 on August 2nd, 2022.

This hierarchical structure allows for easy organization and retrieval of SMS messages based on the contact, date, and time, making it convenient for further analysis or processing.

## Exporting SMS XML

To export SMS messages from an Android device to an XML file, follow these steps:

1. Install the ADB (Android Debug Bridge) tool on your computer. You can download it from [the official Android website](https://developer.android.com/studio/releases/platform-tools)

2. Connect your Android device to your computer using a USB cable.

3. Open a terminal and verify that your device is recognized by running the following command:
```
adb devices
```
> Make sure your device is listed in the output.

4. Backup your SMS messages to an XML file using the following command:
```
adb backup -f sms-backup.ab -noapk com.android.providers.telephony
```
> This command creates a backup file called `sms-backup.ab` in the current directory.

5. Convert the backup file to an XML file using the following command:
```
dd if=sms-backup.ab bs=24 skip=1 | openssl zlib -d | tar -xvf -
```
> This command extracts the contents of the backup file and creates a file called `backup.xml` that contains your SMS messages.

You can then use the SMS Exporter program to convert the XML file to TSV files for further analysis.

## Potential Limitations or Issues

- Performance: Processing large XML files or complex regular expressions can be computationally expensive and may take a significant amount of time.
- Memory Usage: Complex regular expressions may use a lot of memory, which can be a problem on systems with limited resources.
- Pattern Matching: Inaccurate or overly general regular expressions may extract incorrect data from the XML file.
- Character Encoding: Rust's regular expression engine may not handle certain character encodings correctly, which could result in failures or incorrect results.

It's important to test the program thoroughly with different input files and handle errors and edge cases appropriately to ensure proper functionality.

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

## License

These files released under the [MIT License](LICENSE).
