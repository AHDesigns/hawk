import type { Buffer } from "./types";
import { invoke } from "@tauri-apps/api/tauri";

interface Bridge {
  log(...msg: any[]): Promise<void>;
  windowFn(arg: {
    x: number;
    pointInSpace: { x: number; y: number };
  }): Promise<void>;
  openBuffer(arg: { path: string }): Promise<Buffer>;
}

const realBridge: Bridge = {
  log(...msg) {
    return invoke("log", { msg: JSON.stringify(msg) });
  },
  openBuffer({ path }) {
    return invoke("open_buffer", { path });
  },
  windowFn(arg) {
    return invoke("window_fn", arg);
  },
};

const fakeBridge: Bridge = {
  async log(...msg) {
    console.log(JSON.stringify(msg));
  },
  openBuffer() {
    return Promise.resolve({
      name: "package.json",
      lines: [
        "{",
        '  "name": "panda",',
        '  "scripts": {',
        '    "tauri": "tauri",',
        '    "dev": "tauri dev",',
        '    "build": "tauri build",',
        '    "web:dev": "vite",',
        '    "web:build": "vite build",',
        '    "web:serve": "vite preview",',
        '    "ts": "tsc --noEmit"',
        "  },",
        '  "devDependencies": {',
        '    "@tauri-apps/cli": "^1.0.0-beta.10",',
        '    "prettier": "^2.4.1",',
        '    "typescript": "^4.5.2",',
        '    "vite": "^2.5.7",',
        '    "vite-plugin-solid": "^2.0.3"',
        "  },",
        '  "dependencies": {',
        '    "@tauri-apps/api": "^1.0.0-beta.8",',
        '    "solid-js": "^1.1.3"',
        "  }",
        "}",
      ].map((line) => line.split("")),
    });
  },
  async windowFn() {
    console.log("windowFn called");
  },
};

const inTauri = window.__TAURI__;
const bridge: Bridge = inTauri ? realBridge : fakeBridge;
export default bridge;
