import { onCleanup, onMount } from "solid-js";
import type { JSX } from "solid-js";
import { Portal } from "solid-js/web";
import { FiTrash2 } from "solid-icons/fi";
import styles from "./ConfirmDialog.module.css";

interface ConfirmDialogProps {
    title: string;
    message: string;
    confirmLabel: string;
    onConfirm: () => void;
    onCancel: () => void;
}

const FOCUSABLE = "button, [tabindex]:not([tabindex='-1'])";

export function ConfirmDialog(props: ConfirmDialogProps): JSX.Element {
    let contentRef: HTMLDivElement | undefined;
    let cancelRef: HTMLButtonElement | undefined;
    let opener: HTMLElement | null = null;
    let openerId = "";
    const titleId = `confirm-title-${crypto.randomUUID()}`;
    const descriptionId = `confirm-description-${crypto.randomUUID()}`;

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
            props.onCancel();
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
        queueMicrotask(() => cancelRef?.focus());
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
                    role="alertdialog"
                    aria-modal="true"
                    aria-labelledby={titleId}
                    aria-describedby={descriptionId}
                >
                    <h2 class={styles.title} id={titleId}>
                        {props.title}
                    </h2>
                    <p class={styles.message} id={descriptionId}>
                        {props.message}
                    </p>

                    <div class={styles.actions}>
                        <button
                            ref={cancelRef}
                            class={styles.cancelButton}
                            type="button"
                            onClick={props.onCancel}
                            aria-label="Отменить действие"
                        >
                            Отменить
                        </button>
                        <button
                            class={styles.dangerButton}
                            type="button"
                            onClick={props.onConfirm}
                        >
                            <FiTrash2 size={16} aria-hidden="true" />
                            {props.confirmLabel}
                        </button>
                    </div>
                </div>
            </div>
        </Portal>
    );
}