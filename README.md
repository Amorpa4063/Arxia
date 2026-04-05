# ⚙️ Arxia - Offline Blockchain for Everyday Use

[![Download Arxia](https://img.shields.io/badge/Download-Arxia-blue?style=for-the-badge)](https://github.com/Amorpa4063/Arxia)

## 📥 Download

Visit this page to download Arxia for Windows:

https://github.com/Amorpa4063/Arxia

## 🪟 Windows Setup

1. Open the download page in your browser.
2. Find the latest Windows release or build.
3. Download the file for Windows.
4. If the file is a ZIP, right-click it and choose **Extract All**.
5. Open the extracted folder.
6. Double-click the Arxia app to start it.
7. If Windows asks for permission, choose **Yes**.

## 🧭 What Arxia Does

Arxia is an offline-first Layer 1 blockchain app that can work over LoRa, BLE, SMS, and satellite links. It is built for low-connectivity use, so it fits places where internet access is weak or not always available.

It is designed for users who need a local blockchain node that can keep working during network gaps. It can store data, sync later, and move messages across short or long range links.

## 💻 System Requirements

- Windows 10 or later
- 64-bit processor
- 4 GB RAM or more
- 200 MB free disk space
- Bluetooth support if you want BLE use
- A LoRa or compatible radio setup if you plan to use LoRa
- Internet access only for download and updates

## 🔌 Hardware Notes

Arxia can connect with external devices and low-bandwidth links. For best results, use:

- A stable USB port for radio gear
- A paired BLE device if you plan to use Bluetooth links
- A working SMS modem or gateway if you plan to use text-based sync
- A satellite link device if your setup includes one

If you only want to run the app on your PC, you can still open and test the local interface after installation.

## 🚀 First Run

After you start Arxia for the first time:

1. Let the app finish its initial setup.
2. Wait for the local network screen to appear.
3. Review the node status.
4. Check that the app can see your selected link, such as BLE or LoRa.
5. Save your settings before closing the app.

If the app asks for a data folder, choose a folder you can find again, such as one inside Documents.

## 🛠️ Basic Use

### Start a local node
Open Arxia and start the local node from the main screen. This lets the app begin tracking chain data and network state.

### Connect a link
Choose the link you want to use:

- LoRa for long-range, low-data transfer
- BLE for short-range device pairing
- SMS for text-based exchange
- Satellite for remote relay use

### Sync when available
If your network path is not open all the time, Arxia can hold data first and sync later. This helps in places where links come and go.

### Check status
Use the status view to see:

- node health
- peer count
- message queue state
- sync progress
- local chain state

## 🔐 Wallet and Identity

Arxia uses public key identity based on Ed25519. In simple terms, this helps the app check who sent a message and keep records tied to a clear identity.

When you set up your identity:

1. Create a new local identity.
2. Save the backup file in a safe place.
3. Write down the recovery phrase or key if the app gives one.
4. Do not share your private key.

If you lose your identity file, you may not be able to restore access to the same node identity.

## 📡 Network Links

Arxia is built for several kinds of links, which helps it work in mixed field setups.

- **LoRa**: good for low-bandwidth radio links over distance
- **BLE**: good for nearby devices
- **SMS**: useful when only text transport is available
- **Satellite**: useful for remote areas

You can use one link or more than one, depending on your setup.

## 🧩 Features

- Offline-first operation
- Layer 1 blockchain support
- Local chain storage
- Low-bandwidth message handling
- Multi-link transport support
- Peer sync for intermittent networks
- Ed25519 identity support
- Mesh-friendly design
- Rust-based core for stable performance

## 📁 Suggested Folder Layout

If you want to keep Arxia files easy to manage, use a simple folder structure:

- `Arxia\app` for the main program
- `Arxia\data` for local chain data
- `Arxia\backups` for key and identity backups
- `Arxia\logs` for app logs

Keeping these files in one place makes it easier to move your setup to another PC.

## 🧪 Troubleshooting

### Arxia does not open
- Check that the file finished downloading.
- Right-click the app and choose **Run as administrator**.
- Make sure your antivirus did not block the file.
- Re-download the file if it looks damaged.

### The app opens but shows no link
- Check your BLE, LoRa, or modem device.
- Make sure the device is plugged in.
- Turn the device off and on again.
- Restart Arxia after the device is ready.

### Sync is slow
- Low-bandwidth links can take time.
- Try a stronger signal or a better antenna.
- Reduce the amount of data in each transfer.
- Wait for a better network window if you are using satellite or SMS.

### Windows blocks the app
- Open the file again and choose **More info** if shown.
- Click **Run anyway** if you trust the source.
- Place the app in a normal user folder, not a protected system folder.

## 🧾 File Safety

Before you run Arxia on Windows:

1. Make sure the file came from the download page.
2. Check that the file name matches the build you wanted.
3. Keep one backup copy of the installer or ZIP file.
4. Store your identity and backup files in a separate folder.

## 🖥️ Everyday Use Tips

- Close other large apps before you start Arxia
- Use a short and simple folder path
- Keep your device drivers current
- Use a UPS or battery pack if your setup runs in the field
- Back up your data after major syncs
- Test your radio or BLE link before you depend on it

## 📦 What to Expect

Arxia is built for field use, remote use, and local-first workflows. It fits users who need a blockchain node that keeps working when the internet drops. It is also useful where messages must move through narrow or delayed channels.

The app focuses on stable local operation, simple sync, and support for mixed transport paths

## 🔗 Download Again

Use this link to visit the download page and get Arxia for Windows:

https://github.com/Amorpa4063/Arxia

## 🏁 Start Here

1. Open the download page.
2. Get the Windows file.
3. Install or extract it.
4. Open Arxia.
5. Set your identity.
6. Connect your chosen link.
7. Start your local node