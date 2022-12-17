const refreshRate = 1; // milliseconds
const ballSize = 20;
const backgroundColor = "navy";
const lineColor = "red";
const ballColor = "yellow";

class DrawState {
    constructor() {
        this.backgroundColor = backgroundColor;
        this.lineColor = lineColor;
        this.ballColor = ballColor;
        this.newBallStart = null;
        this.newBallHold = null;
    }
}

function draw(canvas, drawState, bolasState) {
    const ctx = canvas.getContext("2d");
    canvas.setAttribute("width", window.innerWidth);
    canvas.setAttribute("height", window.innerHeight);
    ctx.fillStyle = drawState.backgroundColor;
    ctx.strokeStyle = drawState.lineColor;
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = drawState.ballColor;
    ctx.strokeStyle = drawState.ballColor;

    for (b of bolasState.bolas) {
        ctx.beginPath();
        ctx.arc(b.c.x, b.c.y, ballSize, 0, 2* Math.PI);
        ctx.stroke();
        ctx.fill();
    }

    if (drawState.newBallStart != null && drawState.newBallHold != null) {
        ctx.strokeStyle = drawState.lineColor;
        ctx.beginPath();
        ctx.moveTo(drawState.newBallStart.x, drawState.newBallStart.y);
        ctx.lineTo(drawState.newBallHold.x, drawState.newBallHold.y);
        ctx.stroke();
    }
}

function setupCanvasEvents(canvas, drawState, socket) {
    let clickSpot = null;

    canvas.onmousedown = (e) => {
        drawState.newBallStart = e;
    };

    canvas.onmousemove = (e) => {
        drawState.newBallHold = e;
    };

    canvas.onmouseup = (e) => {
        drawState.newBallHold = e;

        if (drawState.newBallStart != null) {
            let velX = Math.floor(drawState.newBallStart.x - e.x);
            let velY = Math.floor(drawState.newBallStart.y - e.y);

            socket.send(JSON.stringify({NewBola: {c: {x: e.x, y: e.y}, v: {vel_x: velX, vel_y: velY}}}));
            drawState.newBallStart = null;
            drawState.newBallHold = null;
        }
    }
}

function setupWebsocketEvents(canvas, drawState) {
    let socket = new WebSocket("ws://localhost:8080/ws");

    socket.onopen = (e) => {
        socket.send(JSON.stringify({SetCanvasDimensions: {height: canvas.height, width: canvas.width}}))
    };

    socket.onmessage = (e) => {
        console.log(`Received data ${e.data}`);
        draw(canvas, drawState, JSON.parse(e.data));
    };

    socket.onclose = (e) => {
        console.log("Socket closed");
    };

    socket.onerror = (e) => {
        console.log("Socket errored");
    };

    return socket;
}


const drawState = new DrawState();
const canvas = document.getElementById("bolas");
draw(canvas, drawState, {bolas: []});
let socket = setupWebsocketEvents(canvas, drawState);
setupCanvasEvents(canvas, drawState, socket);
