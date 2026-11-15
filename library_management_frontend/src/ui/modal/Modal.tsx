import type { JSX } from "solid-js";
import { FiX } from "solid-icons/fi";

import styles from "./Modal.module.css";

interface ModalProps {
    title: string;
    onClose: () => void;
    children: JSX.Element;
}

export function Modal(props: ModalProps) {
    return (
        <div class={styles.overlay} role="presentation" onClick={props.onClose}>
            <div
                class={styles.window}
                role="dialog"
                aria-modal="true"
                aria-label={props.title}
                onClick={(event) => event.stopPropagation()}
            >
                <div class={styles.header}>
                    <h2 class={styles.title}>{props.title}</h2>
                    <button
                        class={styles.closeButton}
                        type="button"
                        onClick={props.onClose}
                        aria-label="Закрыть окно"
                    >
                        <FiX size={18} aria-hidden="true" />
                    </button>
                </div>
                <div class={styles.body}>{props.children}</div>
            </div>
        </div>
    );
}