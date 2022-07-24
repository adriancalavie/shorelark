import * as sim from "lib-simulation-wasm";

let simulation = new sim.Simulation();

document.getElementById('train').onclick = function () {
    console.log(simulation.train());
};

const canvas = document.querySelector("canvas");

const resize = () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
}

resize()
window.addEventListener('resize', resize)
// var canvas;

// window.onload = window.onresize = function () {

//     canvas = document.getElementById('myCanvas');
//     canvas.width = window.innerWidth;
//     canvas.height = window.innerHeight;    
// }

class SimulationRendering2D {

    constructor(canvasRenderingContext2D) {
        this.context = canvasRenderingContext2D
    }

    moveTo(x, y) {
        console.log("Moved to: (" + x + "," + y + ")")
        this.context.moveTo(x, y)
    }
    lineTo(x, y) {
        console.log("Line to: (" + x + "," + y + ")")
        this.context.lineTo(x, y)
    }
    scale(xScale, yScale) {
        this.context.scale(xScale, yScale)
    }
    clearRect(x, y, width, height) {
        this.context.clearRect(x, y, width, height)
    }

    drawTriangle(x, y, size, rotation) {
        this.context.beginPath();

        this.moveTo(
            x + Math.cos(rotation) * size * 1.5,
            y + Math.sin(rotation) * size * 1.5,
        );

        this.lineTo(
            x + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
            y + Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
        );

        this.lineTo(
            x + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
            y + Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
        );

        this.lineTo(
            x + Math.cos(rotation) * size * 1.5,
            y + Math.sin(rotation) * size * 1.5,
        );

        this.context.fillStyle = 'rgb(255, 255, 255)';
        this.context.fill();

        console.log("Rendered triangle")
    }

    drawCircle(x, y, radius) {
        this.context.beginPath()
        // ---
        // | Circle's center.
        // ----- v -v
        this.context.arc(x, y, radius, 0, 2.0 * Math.PI);
        // ------------------- ^ -^-----------^
        // | Range at which the circle starts and ends, in radians.
        // |
        // | By manipulating these two parameters you can e.g. draw
        // | only half of a circle, Pac-Man style.
        // ---

        this.context.fillStyle = 'rgb(0, 255, 128)';
        this.context.fill();
    }
}

const viewportWidth = canvas.width;
const viewportHeight = canvas.height;

const viewportScale = window.devicePixelRatio || 1;
// ------------------------------------------ ^^^^
// | Syntax-wise, it's like: .unwrap_or(1)
// |
// | This value determines how much physical pixels there are per
// | each single pixel on a canvas.
// |
// | Non-HiDPI displays usually have a pixel ratio of 1.0, which
// | means that drawing a single pixel on a canvas will lighten-up
// | exactly one physical pixel on the screen.
// |
// | My display has a pixel ratio of 2.0, which means that for each
// | single pixel drawn on a canvas, there will be two physical
// | pixels modified by the browser.
// ---

// The Trick, part 1: we're scaling-up canvas' *buffer*, so that it
// matches the screen's pixel ratio
canvas.width = viewportWidth * viewportScale;
canvas.height = viewportHeight * viewportScale;

// The Trick, part 2: we're scaling-down canvas' *element*, because
// the browser will automatically multiply it by the pixel ratio in
// a moment.
//
// This might seem like a no-op, but the maneuver lies in the fact
// that modifying a canvas' element size doesn't affect the canvas'
// buffer size, which internally *remains* scaled-up:
//
// ----------- < our entire page
// |         |
// |   ---   |
// |   | | < | < our canvas
// |   ---   |   (size: viewport.style.width & viewport.style.height)
// |         |
// -----------
//
// Outside the page, in the web browser's memory:
//
// ----- < our canvas' buffer
// |   | (size: viewport.width & viewport.height)
// |   |
// -----
canvas.style.width = viewportWidth + 'px';
canvas.style.height = viewportHeight + 'px';

const context = new SimulationRendering2D(canvas.getContext('2d'))

// Automatically scales all operations by `viewportScale` - otherwise
// we'd have to `* viewportScale` everything by hand
context.scale(viewportScale, viewportScale);

// Rest of the code follows without any changes
context.fillStyle = 'rgb(0, 0, 0)';

// for (const animal of simulation.world().animals) {
//     context.drawTriangle(
//         animal.x * viewportWidth,
//         animal.y * viewportHeight,
//         0.01 * viewportWidth,
//         animal.rotation,
//     );
// }

function redraw() {
    context.clearRect(0, 0, viewportWidth, viewportHeight);

    simulation.step();

    const world = simulation.world();

    for (const food of world.foods) {
        context.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHeight,
            (0.01 / 2.0) * viewportWidth,
        )
    }

    for (const animal of world.animals) {
        context.drawTriangle(
            animal.x * viewportWidth,
            animal.y * viewportHeight,
            0.01 * viewportWidth,
            animal.rotation,
        );
    }

    // requestAnimationFrame() schedules code only for the next frame.
    //
    // Because we want for our simulation to continue forever, we've
    // gotta keep re-scheduling our function:
    requestAnimationFrame(redraw);
}

redraw()