const windowResizeDebounceTimeout = 50; // milliseconds
const bolaRadius = 20;
const defaultBackgroundColor = "#046a38";
const defaultBolaLineColor = "#ffe900";
const defaultBolaColor = "#da291c";

class BolasState {
    constructor() {
        this.bolas = [];
        this.bolasUpdated = true;
        this.backgroundColor = defaultBackgroundColor;
        this.bolaLineColor = defaultBolaLineColor;
        this.bolaColor = defaultBolaColor;
        this.newBallStart = null;
        this.newBallHold = null;
        this.lastLineStart = null;
        this.lastLineEnd = null;
    }
}

function drawLoop(canvas, bolasState) {
    draw(canvas, bolasState);
    requestAnimationFrame(() => drawLoop(canvas, bolasState));
}

function drawLine(ctx, start, end, color) {
    ctx.strokeStyle = color;
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
    ctx.stroke();
}

function drawBallLine(ctx, bolasState) {
    drawLine(
        ctx,
        bolasState.newBallStart,
        bolasState.newBallHold,
        bolasState.bolaLineColor,
    );
    bolasState.lastLineStart = bolasState.newBallStart;
    bolasState.lastLineEnd = bolasState.newBallHold;
}

function drawBolas(ctx, bolasState) {
    for (let b of bolasState.bolas) {
        ctx.fillStyle = bolasState.bolaColor;
        ctx.strokeStyle = bolasState.bolaColor;

        ctx.beginPath();
        ctx.arc(b.c.x, b.c.y, bolaRadius, 0, 2 * Math.PI);
        ctx.stroke();
        ctx.fill();
    }

    bolasState.bolasUpdated = false;
}

function fullRedraw(canvas, bolasState) {
    // Set the whole canvas to the background color, clearing everything drawn
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = bolasState.backgroundColor;
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    if (bolasState.newBallStart != null && bolasState.newBallHold != null) {
        drawBallLine(ctx, bolasState);
    }

    drawBolas(ctx, bolasState);
}

function draw(canvas, bolasState) {
    if (bolasState.bolasUpdated) {
        fullRedraw(canvas, bolasState);
        return;
    }

    const ctx = canvas.getContext("2d");

    if (bolasState.lastLineStart != null && bolasState.lastLineEnd != null) {
        drawLine(
            ctx,
            bolasState.lastLineStart,
            bolasState.lastLineEnd,
            bolasState.backgroundColor,
        );
        bolasState.lastLineStart = null;
        bolasState.lastLineEnd = null;
    }

    if (bolasState.newBallStart != null && bolasState.newBallHold != null) {
        drawBallLine(ctx, bolasState);
    }

    drawBolas(ctx, bolasState);
}

function debounce(func) {
    var timer;

    return (e) => {
        if (timer) {
            clearTimeout(timer);
        }

        timer = setTimeout(() => func(e), windowResizeDebounceTimeout);
    };
}

function resizeCanvas(canvas, socket) {
    canvas.setAttribute("height", window.innerHeight);
    canvas.setAttribute("width", window.innerWidth);
    socket.send(
        JSON.stringify({
            SetCanvasDimensions: { height: canvas.height, width: canvas.width },
        }),
    );
}

function setupCanvasEvents(
    canvas,
    bolasState,
    socket,
    downEvent,
    moveEvent,
    upEvent,
    getX,
    getY,
) {
    window.addEventListener(
        "resize",
        debounce((_) => resizeCanvas(canvas, socket)),
    );

    canvas[downEvent] = (e) => {
        bolasState.newBallStart = { x: getX(e), y: getY(e) };
    };

    canvas[moveEvent] = (e) => {
        bolasState.newBallHold = { x: getX(e), y: getY(e) };
    };

    canvas[upEvent] = () => {
        if (bolasState.newBallStart != null) {
            let velX = Math.floor(
                bolasState.newBallStart.x - bolasState.newBallHold.x,
            );
            let velY = Math.floor(
                bolasState.newBallStart.y - bolasState.newBallHold.y,
            );

            socket.send(
                JSON.stringify({
                    NewBola: {
                        c: bolasState.newBallHold,
                        v: { vel_x: velX, vel_y: velY },
                    },
                }),
            );

            bolasState.newBallStart = null;
            bolasState.newBallHold = null;
        }
    };
}

function setupDesktopEvents(canvas, bolasState, socket) {
    console.log("Setting up bolas events for desktop browswer");
    setupCanvasEvents(
        canvas,
        bolasState,
        socket,
        "onmousedown",
        "onmousemove",
        "onmouseup",
        (e) => e.x,
        (e) => e.y,
    );
}

function setupMobileEvents(canvas, bolasState, socket) {
    console.log("Setting up bolas events for mobile browswer");
    setupCanvasEvents(
        canvas,
        bolasState,
        socket,
        "ontouchstart",
        "ontouchmove",
        "ontouchend",
        (e) => e.touches[0].clientX,
        (e) => e.touches[0].clientY,
    );
}

function isMobile() {
    return "ontouchstart" in document.documentElement;
}

function setupWebsocketEvents(canvas, bolasState) {
    let server = location.origin.replace(/^http/, "ws") + "/ws";
    let socket = new WebSocket(server);

    socket.onopen = (_) => {
        resizeCanvas(canvas, socket);

        if (isMobile()) {
            setupMobileEvents(canvas, bolasState, socket);
        } else {
            setupDesktopEvents(canvas, bolasState, socket);
        }

        setupCanvasEvents(canvas, bolasState, socket);
        drawLoop(canvas, bolasState);
    };

    socket.onmessage = (e) => {
        let bolas = JSON.parse(e.data).bolas;
        bolasState.bolas = bolas;
        bolasState.bolasUpdated = true;
    };

    socket.onclose = (_) => {
        console.log("Socket closed");
    };

    socket.onerror = (_) => {
        console.log("Socket errored");
    };

    return socket;
}

const bolasState = new BolasState();
const canvas = document.getElementById("bolas");
const socket = setupWebsocketEvents(canvas, bolasState);
