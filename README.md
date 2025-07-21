
# VERDE-X

VERDE-X is a decentralized software system built with a combination of Rust, TypeScript (TSX), and Python. The project is organized for clarity and high performance, using modern tools like Tauri and secure crypto integrations.

___

## 📂 Project Structure

VERDE-X/ │ ├── src/             # React frontend (TypeScript + TSX) ├── src-tauri/       # Tauri backend using Rust ├── bitcoin/         # Python scripts for Bitcoin integration ├── public/          # Static assets and logos ├── .env             # Environment variables (not uploaded) ├── README.md        # Project documentation

---

## 🛠️ Technologies Used

- **TypeScript + React (TSX)** — Frontend UI
- **Rust** — Backend logic via Tauri
- **Python** — Bitcoin and crypto integration
- **Tauri** — Desktop application shell

---

## 🔧 Setup Instructions

### 1. Clone the repository
```bash
git clone https://github.com/Bill-Dave/VERDE-X.git
cd VERDE-X

2. Install frontend dependencies

npm install

3. Install Rust and Tauri CLI

cargo install tauri-cli

4. Install Python dependencies

pip install -r bitcoin/requirements.txt


---

▶️ Run the App

npm run tauri dev

Make sure your .env file is set up correctly.


---

🔐 .env File Example

REACT_APP_API_KEY=your_api_key
BITCOIN_SECRET=your_bitcoin_key
TAURI_KEY=your_secret_key

> Do not upload your .env file to GitHub.




---

📄 License

MIT © 2025 Bill Dave
