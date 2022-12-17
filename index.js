const refreshRate = 100; // milliseconds

function BolasState() {
    this.newBallStart = null;
    this.newBallHold = null;
}

function draw(canvas, state) {
    const ctx = canvas.getContext("2d");
    canvas.setAttribute("width", window.innerWidth);
    canvas.setAttribute("height", window.innerHeight);
    ctx.fillStyle = "navy";
    ctx.strokeStyle = "red";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    if (state.newBallStart != null && state.newBallHold != null) {
        console.log(`Drawing line from [${state.newBallStart.x}, ${state.newBallStart.y}] -> [${state.newBallHold.x}, ${state.newBallHold.y}]`);
        ctx.beginPath();
        ctx.moveTo(state.newBallStart.x, state.newBallStart.y);
        ctx.lineTo(state.newBallHold.x, state.newBallHold.y);
        ctx.stroke();
    }
}

function drawLooop(canvas, state) {
    draw(canvas, state);
    setTimeout(() => drawLooop(canvas, state), refreshRate);
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
            console.log(`Will release ball [${state.newBallStart.x}, ${state.newBallStart.y}] -> [${state.newBallHold.x}, ${state.newBallHold.y}]`);
            state.newBallStart = null;
            state.newBallHold = null;
        }
    }
}


const state = new BolasState();
const canvas = document.getElementById("bolas");
setupEvents(canvas, state)
drawLooop(canvas, state);
