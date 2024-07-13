import init, { trick_png } from './rust_randomart.js';

async function run() {
    await init();
    setInterval(()=>{
        document.getElementById("mainImg1").src = trick_png();
    }, 10);
    setInterval(()=>{
        document.getElementById("mainImg2").src = trick_png();
    }, 100);
    setInterval(()=>{
        document.getElementById("mainImg3").src = trick_png();
    }, 1000);
}
run();

