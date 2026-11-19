import type { JSX } from "solid-js";

import styles from "./Footer.module.css";

export function Footer(): JSX.Element {
    return (
        <footer class={styles.footer}>
            <span>© {new Date().getFullYear()} Ragnamoris Corp.</span>
        </footer>
    );
}