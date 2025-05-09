// TODO: add an area marker that shows effective area
// TODO: use env variables to set the constants

const startBtn = document.getElementById('startBtn');
const stopBtn = document.getElementById('stopBtn');
const videoEl = document.getElementById('webcam');

/* ==== knobs you can change ==================================== */
const INTERVAL_MS = 3_000;
const MAX_FRAMES = 20;
const CANVAS_SIZE = 512;           // 512 × 512 crop
/* ============================================================== */

let grabTimer = null;
let framesSent = 0;
let stream = null;
let grabber = null;
let canvas, ctx;

/* --- life-cycle helpers --------------------------------------- */
function stopPipeline() {
    clearInterval(grabTimer);
    grabTimer = null;
    stopBtn.disabled = true;
    startBtn.disabled = false;

    if (stream) {
        stream.getTracks().forEach(t => t.stop());
        stream = null;
    }
}

async function startPipeline() {
    startBtn.disabled = true;
    stopBtn.disabled = false;

    framesSent = 0; // reset frame counter

    /* 1. open camera (prefer rear) */
    const constraints = { video: { facingMode: { exact: "environment" } } };
    try { stream = await navigator.mediaDevices.getUserMedia(constraints); }
    catch { stream = await navigator.mediaDevices.getUserMedia({ video: true }); }
    videoEl.srcObject = stream;

    /* 2. prep helpers */
    [grabber] = ("ImageCapture" in window) ? [new ImageCapture(stream.getVideoTracks()[0])] : [null];
    canvas = canvas ?? new OffscreenCanvas(CANVAS_SIZE, CANVAS_SIZE);
    ctx = ctx ?? canvas.getContext("2d");

    /* 3. kick the loop */
    grabTimer = setInterval(grabAndSend, INTERVAL_MS);
    await grabAndSend();           // first frame immediately
}

/* --- main worker ---------------------------------------------- */
async function grabAndSend() {
    /* Guard #1 – max frame budget */
    if (framesSent >= MAX_FRAMES) return stopPipeline();

    /* Guard #2 – skip if tab hidden (user switched away) */
    if (document.hidden) return;

    try {
        const bitmap = grabber ? await grabber.grabFrame()
            : await createImageBitmap(videoEl);

        /* square-crop & resize */
        const [bw, bh] = [bitmap.width, bitmap.height];
        const side = Math.min(bw, bh);
        const sx = (bw - side) / 2, sy = (bh - side) / 2;

        ctx.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
        ctx.drawImage(bitmap, sx, sy, side, side, 0, 0, CANVAS_SIZE, CANVAS_SIZE);

        /* encode + upload */
        const blob = await canvas.convertToBlob({
            type: "image/webp",
            quality: 1.0
        });

        await fetch("/api/frame", {
            method: "POST",
            body: blob,
            headers: { "Content-Type": "image/webp" }
        });

        framesSent++;
    } catch (e) {
        console.error("grab/send failed:", e);
        /* optional: add exponential back-off here */
    }
}

/* --- wiring ---------------------------------------------------- */
startBtn.addEventListener('click', startPipeline);
stopBtn.addEventListener('click', stopPipeline);

/* free the camera if the user closes or reloads the page */
addEventListener("beforeunload", stopPipeline);