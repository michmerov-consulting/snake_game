import init, { Direction, GameStatus, World } from "snake_game";
import { wasm } from "webpack";
import {rnd} from "./utils/rnd.js";
init().then(wasm => {
    const CELL_SIZE = 20;
    const WORLD_WIDTH = 16;
    const gamePoints  = <HTMLDivElement> document.getElementById("game-points");

    const gameControlBtn  = <HTMLButtonElement> document.getElementById("game-control-btn");
    const gameStatusText = <HTMLDivElement> document.getElementById("game-status");
    const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH);
    const snakeDirection = rnd(4);
    const snakeSize = 3;
    const world = World.new(WORLD_WIDTH, snakeSpawnIdx, snakeDirection, snakeSize);
    const worldWidth = world.width();
    const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");
    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;
    gameControlBtn.addEventListener("click", e =>{
        const gameStatus = world.get_game_status();
        if(gameStatus === undefined){
            gameControlBtn.textContent = "Playing ...";
            world.change_game_status();
            play();
        }else{
            location.reload();
        }
        
            
    });
    document.addEventListener("keydown", (e) => {
        switch (e.code) {
            case "ArrowUp":
                console.debug("change to arrow up")
                world.change_snake_direction(Direction.Up);
                break;
            case "ArrowRight":
                console.debug("change to arrow right")
                world.change_snake_direction(Direction.Right);
                break;
            case "ArrowDown":
                console.debug("change to arrow down")
                world.change_snake_direction(Direction.Down);
                break;
            case "ArrowLeft":
                console.debug("change to arrow left")
                world.change_snake_direction(Direction.Left);
                break;
        }
    });
    function drawWorld() {
        ctx.beginPath();
        for (let x = 0; x < worldWidth + 1; x++) {
            ctx.moveTo(CELL_SIZE * x, 0);
            ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
        }
        for (let y = 0; y < worldWidth + 1; y++) {
            ctx.moveTo(0, CELL_SIZE * y);
            ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
        }
        ctx.stroke();
    }
    function drawReward() {
        paintCell(world.reward_cell(), -1);
        ctx.stroke();
    }
    function drawSnake() {
        const snakeCells = new Uint32Array(wasm.memory.buffer, world.snake_cells(), world.snake_length());
        
        snakeCells.filter((cell,i) =>!(i >0 && cell === snakeCells[0])).forEach((cell,i) =>{

            paintCell(cell, i);
        });
        
        ctx.stroke();
    }
    function paintCell(snakeIndex:number, idx:number){
        const col = snakeIndex % worldWidth;
        const row = Math.floor(snakeIndex / worldWidth);
        if(idx ===0){
            ctx.fillStyle = "#7878db";
        }else if (idx === -1){
            ctx.fillStyle = "#888888";
        }else{
            ctx.fillStyle = "#000000";
        }
        
        ctx.beginPath();
        ctx.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        );
        
    }
    function drawGameStatus(){
        const status = world.get_game_status();
        gameStatusText.textContent = world.get_game_status_label();
        gamePoints.textContent = world.points().toString(10);
        if(status == GameStatus.Won || status == GameStatus.Lost){
            gameControlBtn.textContent = "Replay?";
        }
    }
    function paint() {
        drawWorld();
        drawSnake();
        drawReward();
        drawGameStatus();
    }

    function play() {
        let fps = 3;
        const status = world.get_game_status();
        if(status != GameStatus.Played) return;
        setTimeout(() => {
            console.debug("updating");
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            world.step();
            paint();
            requestAnimationFrame(play);
        }, 1000 / fps)
    }
    paint();
});