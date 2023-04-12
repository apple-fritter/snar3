# androidmessages.tsv
This program takes an input `XML` file that contains SMS messages as input, and then parses the file using the `xml-rs` crate to extract the relevant information, such as the `sender`, `receiver`, `timestamp`, and `message text`. This program provides a convenient way to export SMS messages from an `XML` file to `TSV` files in a structured and organized format that makes it easy to search, sort, and analyze the data.

It then groups the messages by `contact`, `year`, `month`, and `day`, and exports them to `TSV` files in nested directories based on this grouping. Each `TSV` file contains the messages for a specific day, and has columns for the `sender`, `timestamp` (formatted as `HH:MM:SS`), and message text.

The program can handle both incoming and outgoing messages and indicates which is which by adding `US` to outgoing messages and `THEM` to incoming messages in the sender column.

Here's an example of what one of the resulting TSV files might look like:
```
THEM    08  23  04      Hello, how are you?
US     08  24  31      I'm good, thanks. How about you?
THEM    08  25  17      Doing well, thanks.
```

To run the program, navigate to your project folder in the terminal and run the following command:
```
cargo run -- input_file_path output_dir_path
```

Replace `input_file_path` with the path to your SMS backup file, and `output_dir_path` with the path to the directory where you want to save the exported files.

For example, if your SMS backup file is located in your home folder and you want to save the exported files in a folder called exports on your desktop, you can run the following command:
```
cargo run -- ~/sms-backup.xml ~/Desktop/exports
```

# Export the SMS messages to an XML file

* Install the ADB tool on your computer.
* You can download the tool from the official Android website: https://developer.android.com/studio/releases/platform-tools

* Connect your device to your computer using a USB cable.

* Open a terminal or command prompt on your computer and navigate to the folder where you installed the ADB tool.
---
Type the following command to check if your device is connected and recognized by the ADB tool:
`adb devices`

If your device is recognized, you will see its name in the output of the command. If your device is not recognized, make sure that USB debugging is enabled on your device by going to `"Settings" > "Developer options" > "USB debugging".`

Type the following command to backup your SMS messages to an XML file:
`adb backup -f sms-backup.ab -noapk com.android.providers.telephony`

This command will create a backup file called `sms-backup.ab` in the folder where you executed the command. The `-noapk` option tells `ADB` not to backup the `APK` files, and `com.android.providers.telephony` is the package name for the SMS app on your device.

Once the backup is complete, type the following command to convert the backup file to an `XML` file:
`dd if=sms-backup.ab bs=24 skip=1 | openssl zlib -d | tar -xvf -`

This command will extract the contents of the backup file to the current directory and create a file called `backup.xml` that contains your SMS messages.

You can then use this Rust program to export your SMS messages to TSV files. Just replace "input_file_path" with the path to the "backup.xml" file and "output_dir_path" with the path to the directory where you want to save the exported files.
