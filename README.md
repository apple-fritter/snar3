# androidmessages.tsv
This program takes an input `XML` file that contains SMS messages as input, and then parses the file using the `xml-rs` crate to extract the relevant information, such as the `sender`, `receiver`, `timestamp`, and `message text`. This program provides a convenient way to export SMS messages from an `XML` file to `TSV` files in a structured and organized format that makes it easy to search, sort, and analyze the data.

It then groups the messages by `contact`, `year`, `month`, and `day`, and exports them to TSV files in nested directories based on this grouping. Each TSV file contains the messages for a specific day, and has columns for the `sender`, `timestamp` (formatted as `HH:MM:SS`), and message text.

The program can handle both incoming and outgoing messages and indicates which is which by adding `US` to outgoing messages and `THEM` to incoming messages in the sender column.

Here's an example of what one of the resulting TSV files might look like:
```
THEM    08  23  04      Hello, how are you?
US     08  24  31      I'm good, thanks. How about you?
THEM    08  25  17      Doing well, thanks.
```
## Requirements
Apart from Rust, the program requires a few dependencies that need to be installed. These dependencies are specified in the `Cargo.toml` file and include:
* `xml-rs`: This is a crate for parsing XML files. It is used to parse the SMS database file in XML format.
* `chrono`: This is a crate for working with dates and times. It is used to convert the timestamps in the SMS database file into a human-readable format.
* `regex`: This is a crate for working with regular expressions. It is used to extract the SMS messages from the XML file.

When you run `cargo build` or `cargo run`, `Cargo` will automatically download and install these dependencies if they are not already installed on your system.

Apart from these dependencies, the program requires access to the SMS database file on your Android device. You can copy this file to your computer using `Android Debug Bridge (ADB)` or a file transfer tool. The program assumes that the SMS database file is located at `/sdcard/sms/sms-YYYY-MM-DD.xml`, where `YYYY-MM-DD` is the date of the SMS database backup. If your SMS database file is located at a different path or has a different name, you will need to modify the program accordingly.

## Usage
To run the program, navigate to your project folder in the terminal and run the following command:
```
cargo run -- input_file_path output_dir_path
```

Replace `input_file_path` with the path to your SMS backup file, and `output_dir_path` with the path to the directory where you want to save the exported files.

For example, if your SMS backup file is located in your home folder and you want to save the exported files in a folder called exports on your desktop, you can run the following command:
```
cargo run -- ~/sms-backup.xml ~/Desktop/exports
```

## Export SMS messages to an XML file

### Install the ADB tool on your computer.
You can download the tool from the official Android website: https://developer.android.com/studio/releases/platform-tools

* Connect your device to your computer using a USB cable.

* Open a terminal and type the following command to check if your device is connected and recognized by the ADB tool:
`adb devices`

----
If your device is recognized, you will see its name in the output of the command. If your device is not recognized, make sure that USB debugging is enabled on your device by going to `Settings > Developer options > USB debugging`.

Type the following command to backup your SMS messages to an XML file:

`adb backup -f sms-backup.ab -noapk com.android.providers.telephony`

This command will create a backup file called `sms-backup.ab` in the folder where you executed the command. The `-noapk` option tells `ADB` not to backup the `APK` files, and `com.android.providers.telephony` is the package name for the SMS app on your device.

Once the backup is complete, type the following command to convert the backup file to an XML file:

`dd if=sms-backup.ab bs=24 skip=1 | openssl zlib -d | tar -xvf -`

This command will extract the contents of the backup file to the current directory and create a file called `backup.xml` that contains your SMS messages.

You can then use this Rust program to export your SMS messages to TSV files. Just replace `input_file_path` with the path to the `backup.xml` file and `output_dir_path` with the path to the directory where you want to save the exported files.

## Potential limitations or issues
There are some potential limitations or issues to keep in mind when working with regular expressions in Rust:
* Performance: Regular expressions can be computationally expensive, especially when working with large amounts of data. Depending on the size of the XML file and the complexity of the regular expressions used, the program may take a significant amount of time to run.
* Memory usage: Regular expressions can also use a lot of memory, particularly when dealing with complex patterns. This could cause problems if the program needs to run on a machine with limited memory.
* Pattern matching: The program relies on regular expressions to extract information from the XML file. If the regular expressions are not properly designed or are too general, they may match patterns that were not intended, leading to incorrect data being extracted.
* Character encoding: The XML file may contain characters that are not properly handled by Rust's regular expression engine. If this is the case, the program may fail or produce incorrect results.

It's important to thoroughly test this program with various input files and to handle errors and edge cases appropriately to ensure that it works as intended. Please message me with your suggestions!
