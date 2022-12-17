const refreshRate = 100; // milliseconds
const ballSize = 20; // milliseconds

function BolasState() {
    this.backgroundColor = "navy";
    this.lineColor = "red";
    this.ballColor = "yellow";
    this.newBallStart = null;
    this.newBallHold = null;
    this.bolas = [];
}

function Bola(posX, posY, velX, velY) {
    this.posX = posX;
    this.posY = posY;
    this.velX = velX;
    this.velX = velX;
}

function frame(canvas, state) {
    const ctx = canvas.getContext("2d");
    canvas.setAttribute("width", window.innerWidth);
    canvas.setAttribute("height", window.innerHeight);
    ctx.fillStyle = state.backgroundColor;
    ctx.strokeStyle = state.lineColor;
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = state.ballColor;
    ctx.strokeStyle = state.ballColor;

    for (b of state.bolas) {
        //b.posX += b.velX;
        //b.posY += b.velY;
        ctx.beginPath();
        ctx.arc(b.posX, b.posY, ballSize, 0, 2* Math.PI);
        ctx.stroke();
        ctx.fill();
    }

    if (state.newBallStart != null && state.newBallHold != null) {
        ctx.strokeStyle = state.lineColor;
        ctx.beginPath();
        ctx.moveTo(state.newBallStart.x, state.newBallStart.y);
        ctx.lineTo(state.newBallHold.x, state.newBallHold.y);
        ctx.stroke();
    }
}

function frameLooop(canvas, state) {
    frame(canvas, state);
    setTimeout(() => frameLooop(canvas, state), refreshRate);
}

function setupEvents(canvas, state) {
    let clickSpot = null;

    canvas.onmousedown = (e) => {
        state.newBallStart = e;
    };

    canvas.onmousemove = (e) => {
        state.newBallHold = e;
    };

    canvas.onmouseup = (e) => {
        state.newBallHold = e;

        if (state.newBallStart != null) {
            state.bolas.push(new Bola(e.x, e.y, e.x - state.newBallStart.x, e.y - state.newBallStart.y));
            state.newBallStart = null;
            state.newBallHold = null;
        }
    }
}


const state = new BolasState();
const canvas = document.getElementById("bolas");
setupEvents(canvas, state)
frameLooop(canvas, state);
