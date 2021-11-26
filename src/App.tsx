import { Component, createSignal, Index, Show } from "solid-js";

import styles from "./App.module.css";
import b from "./bridge";
import { Buffer } from "./types";

b.windowFn({ x: 3, pointInSpace: { x: 3, y: 4 } });

const App: Component = () => {
  const [buffer, setBuffer] = createSignal<null | Buffer>(null);

  function openFile() {
    b.openBuffer({ path: "/Users/adh23/dev/panda/package.json" })
      .then((myb) => {
        b.log(myb.lines);
        setBuffer(myb);
      })
      .catch((e) => alert(e));
  }

  const w = Object.keys(window);

  return (
    <div class={styles.App}>
      <Show when={window.__TAURI__}>{() => <div>in tauri</div>}</Show>
      <Index each={w}>{(l) => <p>{l}</p>}</Index>
      <button onClick={openFile}>open file</button>
      <Show when={buffer()}>{(b) => <EditableBuffer content={b} />}</Show>
    </div>
  );
};

const EditableBuffer: Component<{ content: Buffer }> = ({ content }) => {
  return (
    <div class={styles.buffer}>
      <Index each={content.lines}>
        {(item) => (
          <div>
            <code>{item()}</code>
          </div>
        )}
      </Index>
    </div>
  );
};

export default App;
