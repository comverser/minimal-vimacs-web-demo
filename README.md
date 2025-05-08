# ⚡ Minimal Vimacs Web Demo

This lightweight web demo is designed for real-time logging of critical medical activities, such as CPR and intubation. It streams a live webcam feed, uses AI to analyze captured images, and continuously logs procedure labels in real time.

---

## ⚠️ Important Notices

- **Single-User Limitation**:  
  Only one user can access and use this demo application at a time. Access will be temporarily unavailable to other users when someone is actively using the demo.

- **Optimal Image Capture**:  
  For optimal AI accuracy under limited conditions, capture images from as close as possible to the subject, maximizing image clarity and enhancing detection quality.

---

## 🔧 Configuration

### Secrets.toml

Before starting the application, create a `Secrets.toml` file in the project root with the following entries:

```toml
OPENAI_API_KEY = "your_openai_api_key_here"
AUTH = "your_auth_token_here"  # Set to 'TEST' to mock API calls for testing
```

- **OPENAI_API_KEY**: Your private API key from OpenAI.
- **AUTH**: Your authentication token. Use `TEST` for test mode, bypassing live API calls and generating mock responses.

### Client-Side Configuration (script.js)

Adjust client behavior by editing these constants in `assets/script.js`:

```javascript
const INTERVAL_MS = 3000;  // Interval between frame captures (milliseconds)
const MAX_FRAMES = 20;     // Maximum frames per session
```

---

## 🚀 How It Works

- **Live Webcam Feed:**  
  Displays a live video from your webcam. Control it with the **Start AI Logging** and **Stop AI Logging** buttons.

- **AI-Generated Procedure Log:**  
  Frames are sent to an AI service, returning labels (e.g., CPR, intubation), which are logged in real-time.

- **Test Mode:**  
  Setting `AUTH = "TEST"` bypasses real API calls, simulating responses to save costs during development.

- **Responsive & Real-Time Updates:**  
  Uses server-rendered Maud templates and HTMX for seamless, real-time log updates.

---

## 🚦 Getting Started

1. **Configure Secrets:**

   Create and populate `Secrets.toml` as shown above.

2. **Run the Application:**

   Start the demo using:

   ```bash
   shuttle run --bacon
   ```

---

## 📁 Directory Structure

- **src/app/controllers/home.rs:**  
  Logic for processing frames, API calls, and log updates.

- **src/app/views/components.rs:**  
  HTML templates using Maud, rendering the webcam feed and logs.

- **assets/script.js:**  
  Handles webcam frame capturing, image processing, and real-time interaction.
