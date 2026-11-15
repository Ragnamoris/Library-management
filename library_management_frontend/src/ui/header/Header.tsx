import type { JSX } from "solid-js";

import styles from "./Header.module.css";

export function Header(): JSX.Element {
    return (
        <header class={styles.header}>
            <span class={styles.title}>Городская библиотека им. Е. В. Беляева</span>
        </header>
    );
}