const startBtn = document.getElementById('startBtn');
const videoEl = document.getElementById('webcam');


document.addEventListener("htmx:load", () => {
    console.log("HTMX has been loaded and is handling interactions.");
});

startBtn.addEventListener('click', async () => {
    startBtn.disabled = true;

    const constraints = {
        video: {
            facingMode: { exact: "environment" }
        }
    };

    try {
        const stream = await navigator.mediaDevices.getUserMedia(constraints);
        videoEl.srcObject = stream;

    } catch (err) {
        console.warn("Rear cam not available, falling back to any camera", err);

        try {
            const fallback = await navigator.mediaDevices.getUserMedia({ video: true });
            videoEl.srcObject = fallback;
        } catch (err) {
            console.error(err);
            startBtn.disabled = false;
            return;
        }
    }
});
