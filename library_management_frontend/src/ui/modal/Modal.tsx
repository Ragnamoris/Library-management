import { onCleanup, onMount } from "solid-js";
import type { JSX } from "solid-js";
import { Portal } from "solid-js/web";
import { FiX } from "solid-icons/fi";
import styles from "./Modal.module.css";

interface ModalProps {
    title: string;
    onClose: () => void;
    children: JSX.Element;
}

const FOCUSABLE = "input, select, textarea, button, [tabindex]:not([tabindex='-1'])";

export function Modal(props: ModalProps): JSX.Element {
    let contentRef: HTMLDivElement | undefined;
    let opener: HTMLElement | null = null;
    let openerId = "";
    const titleId = `modal-title-${crypto.randomUUID()}`;

    function focusableNodes(): HTMLElement[] {
        if (!contentRef) {
            return [];
        }
        return Array.from(contentRef.querySelectorAll<HTMLElement>(FOCUSABLE)).filter(
            (node) => !node.hasAttribute("disabled"),
        );
    }

    function handleKeyDown(event: KeyboardEvent): void {
        if (event.key === "Escape") {
            event.preventDefault();
            props.onClose();
            return;
        }
        if (event.key !== "Tab" || !contentRef) {
            return;
        }
        const nodes = focusableNodes();
        if (nodes.length === 0) {
            event.preventDefault();
            return;
        }
        const first = nodes[0];
        const last = nodes[nodes.length - 1];
        if (!contentRef.contains(document.activeElement)) {
            event.preventDefault();
            first.focus();
            return;
        }
        if (event.shiftKey && document.activeElement === first) {
            event.preventDefault();
            last.focus();
        } else if (!event.shiftKey && document.activeElement === last) {
            event.preventDefault();
            first.focus();
        }
    }

    onMount(() => {
        opener = document.activeElement instanceof HTMLElement ? document.activeElement : null;
        openerId = opener?.id ?? "";
        document.body.style.overflow = "hidden";
        document.addEventListener("keydown", handleKeyDown);
        queueMicrotask(() => {
            contentRef?.querySelector<HTMLElement>("input, select, textarea")?.focus();
        });
    });

    onCleanup(() => {
        document.removeEventListener("keydown", handleKeyDown);
        document.body.style.overflow = "";
        const target =
            opener && opener.isConnected
                ? opener
                : openerId
                  ? document.getElementById(openerId)
                  : null;
        target?.focus();
    });

    return (
        <Portal>
            <div class={styles.overlay}>
                <div
                    ref={contentRef}
                    class={styles.content}
                    role="dialog"
                    aria-modal="true"
                    aria-labelledby={titleId}
                >
                    <div class={styles.header}>
                        <h2 class={styles.title} id={titleId}>
                            {props.title}
                        </h2>
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
        </Portal>
    );
}