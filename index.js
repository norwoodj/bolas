const refreshRate = 1; // milliseconds
const ballSize = 20;
const velocityFactor = 64;
const backgroundColor = "navy";
const lineColor = "red";
const ballColor = "yellow";

class BolasState {
    constructor() {
        this.backgroundColor = backgroundColor;
        this.lineColor = lineColor;
        this.ballColor = ballColor;
        this.newBallStart = null;
        this.newBallHold = null;
        this.bolas = [];
    }
}

class Bola {
    constructor(startX, endX, startY, endY) {
        this.posX = endX;
        this.velX = (startX - endX) / velocityFactor;
        this.posY = endY;
        this.velY = (startY - endY) / velocityFactor;
    }

    updatePosition(canvas) {
        let newPosX = this.posX + this.velX;
        let newPosY = this.posY + this.velY;

        if (newPosX < 0) {
            newPosX = -newPosX;
            this.velX = -this.velX;
        }
        if (newPosY < 0) {
            newPosY = -newPosY;
            this.velY = -this.velY;
        }

        if (newPosX > canvas.width) {
            newPosX = canvas.width - (newPosX - canvas.width);
            this.velX = -this.velX;
        }

        if (newPosY > canvas.height) {
            newPosY = canvas.height - (newPosY - canvas.height);
            this.velY = -this.velY;
        }

        this.posX = newPosX;
        this.posY = newPosY;
    }
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
        b.updatePosition(canvas);
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
            state.bolas.push(new Bola(state.newBallStart.x, e.x, state.newBallStart.y, e.y));
            state.newBallStart = null;
            state.newBallHold = null;
        }
    }
}


const state = new BolasState();
const canvas = document.getElementById("bolas");
setupEvents(canvas, state)
frameLooop(canvas, state);
