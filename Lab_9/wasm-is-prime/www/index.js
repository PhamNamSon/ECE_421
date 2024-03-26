import * as wasm from "wasm-is-prime";
const textbox1 = document.getElementById("PrimeNumber");

document.getElementById("CheckNumber").addEventListener("click", event => {
    const answer = wasm.check_prime(textbox1.value);
    drawAnswer(answer);
});

const canvas = document.getElementById("board");
const ctx = canvas.getContext("2d");
function drawAnswer(yn) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    let xpos = canvas.width / 2;
    let ypos = canvas.height / 2 + 25;

    ctx.font = '50pt Calibri';
    ctx.fillStyle = 'white';
    ctx.textAlign = 'center';
    if (yn == 0) {
        ctx.fillText('❌', xpos, ypos);
    } else {
        ctx.fillText('✅', xpos, ypos);
    }
}
