import { A } from "@solidjs/router";
import type { JSX } from "solid-js";
import { FiBookOpen, FiCalendar, FiFeather, FiUsers } from "solid-icons/fi";

import styles from "./Sidebar.module.css";

export function Sidebar(): JSX.Element {
    return (
        <aside class={styles.aside}>
            <div class={styles.navTitle}>Меню</div>

            <button class={styles.navButton} type="button">
                <A href="/publishers" class={styles.navLink} activeClass={styles.navLinkActive}>
                    <FiFeather size={18} aria-hidden="true" />
                    Издательства
                </A>
            </button>

            <button class={styles.navButton} type="button">
                <A href="/books" class={styles.navLink} activeClass={styles.navLinkActive}>
                    <FiBookOpen size={18} aria-hidden="true" />
                    Книги
                </A>
            </button>

            <button class={styles.navButton} type="button">
                <A href="/readers" class={styles.navLink} activeClass={styles.navLinkActive}>
                    <FiUsers size={18} aria-hidden="true" />
                    Читатели
                </A>
            </button>

            <button class={styles.navButton} type="button">
                <A href="/loans" class={styles.navLink} activeClass={styles.navLinkActive}>
                    <FiCalendar size={18} aria-hidden="true" />
                    Выдачи
                </A>
            </button>
        </aside>
    );
}