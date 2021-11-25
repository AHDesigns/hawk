import {
  WindowManager,
  LogicalPosition,
  LogicalSize,
} from "@tauri-apps/api/window";

const w = new WindowManager("main");

w.setPosition(new LogicalPosition(0, 0)).then(() => {
  w.setSize(new LogicalSize(640, 1200));
});
