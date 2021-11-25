import { invoke } from "@tauri-apps/api/tauri";

export function log(...msg: any[]): Promise<void> {
  return invoke("log", { msg: JSON.stringify(msg) });
}
