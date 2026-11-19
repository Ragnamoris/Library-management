import type { RouteSectionProps } from "@solidjs/router";
import type { JSX } from "solid-js";

import styles from "./Root.module.css";
import { Footer } from "./ui/footer/Footer";
import { Header } from "./ui/header/Header";
import { Main } from "./ui/main/Main";
import { Sidebar } from "./ui/sidebar/Sidebar";

export function Root(props: RouteSectionProps): JSX.Element {
    return (
        <section class={styles.appContainer}>
            <Header />
            <Sidebar />
            <Main>{props.children}</Main>
            <Footer />
        </section>
    );
}