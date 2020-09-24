// *************************************************************
//  迷路表示
//
//  2020/8/21 konao
// *************************************************************

import * as wasm from "maze";
import { memory } from "maze/maze_bg";

// +++++++++++++++++++++++++++++++++++++++++
//  1個のセルのサイズを変えたいときはここを修正
// +++++++++++++++++++++++++++++++++++++++++
const CELL_SIZE = 7;

// 迷路オブジェクト生成
const maze = wasm.Maze.new();

// 幅と高さを得る
const width = maze.width();
const height = maze.height();

// canvas準備
const canvas = document.getElementById("mycanvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;
const ctx = canvas.getContext("2d");

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = "#CCCCCC";

    // 縦線
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // 横線
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

const getIndex = (row, column) => {
    return row * width + column;
};

const drawMap = () => {
    const cellsPtr = maze.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width*height);

    ctx.beginPath();

    for (let row=0; row<height; row++) {
        for (let col=0; col<width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = (cells[idx] === wasm.Cell.Wall) ? "#000000" : "#ffffff";

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

const renderLoop = () => {
    drawGrid();
    drawMap();

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
