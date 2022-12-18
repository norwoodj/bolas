const windowResizeDebounceTimeout = 50; // milliseconds
const bolaRadius = 20;
const defaultBackgroundColor = "navy";
const defaultBolaLineColor = "red";
const defaultBolaColor = "yellow";
const defaultBolaCollisionColor = "yellow";

class BolasState {
    constructor() {
        this.bolas = [];
        this.bolasUpdated = true;
        this.backgroundColor = defaultBackgroundColor;
        this.bolaLineColor = defaultBolaLineColor;
        this.bolaColor = defaultBolaColor;
        this.bolaCollisionColor = defaultBolaCollisionColor;
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
        bolasState.bolaLineColor
    );
    bolasState.lastLineStart = bolasState.newBallStart;
    bolasState.lastLineEnd = bolasState.newBallHold;
}

function drawBolas(ctx, bolasState) {
    for (let b of bolasState.bolas) {
        if (b.t > 0) {
            ctx.fillStyle = bolasState.bolaCollisionColor;
            ctx.strokeStyle = bolasState.bolaCollisionColor;
        } else {
            ctx.fillStyle = bolasState.bolaColor;
            ctx.strokeStyle = bolasState.bolaColor;
        }

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
            bolasState.backgroundColor
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
        })
    );
}

function setupCanvasEvents(canvas, bolasState, socket) {
    window.addEventListener(
        "resize",
        debounce((e) => resizeCanvas(canvas, socket))
    );

    canvas.onmousedown = (e) => {
        bolasState.newBallStart = e;
    };

    canvas.onmousemove = (e) => {
        bolasState.newBallHold = e;
    };

    canvas.onmouseup = (e) => {
        bolasState.newBallHold = e;

        if (bolasState.newBallStart != null) {
            let velX = Math.floor(bolasState.newBallStart.x - e.x);
            let velY = Math.floor(bolasState.newBallStart.y - e.y);

            socket.send(
                JSON.stringify({
                    NewBola: {
                        c: { x: e.x, y: e.y },
                        v: { vel_x: velX, vel_y: velY },
                    },
                })
            );
            bolasState.newBallStart = null;
            bolasState.newBallHold = null;
        }
    };
}

function setupWebsocketEvents(canvas, bolasState) {
    let socket = new WebSocket("ws://localhost:8080/ws");

    socket.onopen = (e) => {
        resizeCanvas(canvas, socket);
        setupCanvasEvents(canvas, bolasState, socket);
        drawLoop(canvas, bolasState);
    };

    socket.onmessage = (e) => {
        let bolas = JSON.parse(e.data).bolas;
        bolasState.bolas = bolas;
        bolasState.bolasUpdated = true;
    };

    socket.onclose = (e) => {
        console.log("Socket closed");
    };

    socket.onerror = (e) => {
        console.log("Socket errored");
    };

    return socket;
}

const bolasState = new BolasState();
const canvas = document.getElementById("bolas");
const socket = setupWebsocketEvents(canvas, bolasState);
