import {
  Component,
  createEffect,
  createResource,
  createSignal,
  Show,
} from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

import logo from "./logo.svg";
import styles from "./App.module.css";

const App: Component = () => {
  const [data, setData] = createSignal(1);
  const [finished, setFinished] = createSignal(false);

  createEffect(() => {
    setTimeout(() => {
      setFinished(true);
    }, 1000);
  });

  createResource(data, (d) => {
    return finished()
      ? Promise.resolve(0)
      : invoke("my_custom_command", { num: d }).then((d) =>
          setData(d as number)
        );
  });

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          class={styles.link}
          href="https://github.com/solidjs/solid"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Solid {data()}
        </a>
        <Show when={finished()}>rendered {data()} updates in 1 second</Show>
      </header>
    </div>
  );
};

export default App;
