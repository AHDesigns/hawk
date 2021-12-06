import { Component, createSignal, Index, Match, Show, Switch } from "solid-js";

import styles from "./App.module.css";
import b from "./bridge";
import { Buffer } from "./types";

b.windowFn({ x: 3, pointInSpace: { x: 3, y: 4 } });

const App: Component = () => {
  const [buffer, setBuffer] = createSignal<null | Buffer>(null);
  const [usingMinibuffer, setUsingMinibuffer] = createSignal(true);

  function openFile() {
    b.openBuffer({ path: "/Users/adh23/dev/hawk/package.json" })
      .then((myb) => {
        b.log(myb.lines);
        setBuffer(myb);
      })
      .catch((e) => alert(e));
  }

  return (
    <div class={styles.App}>
      <div class={styles.main}>
        <Show when={window.__TAURI__}>{() => <div>in tauri</div>}</Show>
        <button onClick={openFile}>open file</button>
        <button
          onClick={() => {
            setUsingMinibuffer((c) => !c);
          }}
        >
          toggle minibuffer
        </button>
        <Show when={buffer()}>{(b) => <EditableBuffer content={b} />}</Show>
      </div>
      <Show when={usingMinibuffer()}>
        <div class={styles.minibuffer}>
          <Index each={candiates}>{(i) => <p>{i}</p>}</Index>
        </div>
      </Show>
    </div>
  );
};

const candiates = [
  "height: 100vh;",
  "text-align: center;",
  "background-color: var(--c-bg);",
  "color: var(--text);",
  "display: flex;",
  "flex-direction: column;",
];

const EditableBuffer: Component<{ content: Buffer }> = ({ content }) => {
  return (
    <div class={styles.buffer}>
      <Index each={content.lines}>
        {(line) => (
          <div>
            <Index each={line()}>
              {(cell) => (
                <span class="cell">
                  <Switch fallback={cell()}>
                    <Match when={cell() === " "}>&nbsp;</Match>
                  </Switch>
                </span>
              )}
            </Index>
          </div>
        )}
      </Index>
    </div>
  );
};

export default App;
