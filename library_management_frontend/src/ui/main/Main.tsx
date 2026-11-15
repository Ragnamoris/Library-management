import type { JSX } from "solid-js";

import styles from "./Main.module.css";

interface MainProps {
    children: JSX.Element;
}

export function Main(props: MainProps): JSX.Element {
    return <main class={styles.main}>{props.children}</main>;
}