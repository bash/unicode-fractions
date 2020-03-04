import { loadWasm } from './wasm-loading.js';
import { bindUserControls } from './ui.js';

loadWasm().then(bindUserControls);
