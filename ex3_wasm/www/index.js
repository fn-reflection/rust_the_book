'use strict'

const wasm_modules = import('../pkg').catch(console.error);
Promise.all([wasm_modules]).then(async ([{ draw_mandelbrot_set }]) => {
    document.getElementById('render').addEventListener('click', draw_mandelbrot_set);
});