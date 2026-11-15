import { Modal } from "./Modal";

import styles from "./ConfirmDialog.module.css";

interface ConfirmDialogProps {
    title: string;
    message: string;
    confirmLabel: string;
    onConfirm: () => void;
    onCancel: () => void;
}

export function ConfirmDialog(props: ConfirmDialogProps) {
    return (
        <Modal title={props.title} onClose={props.onCancel}>
            <p class={styles.message}>{props.message}</p>
            <div class={styles.actions}>
                <button class={styles.cancelButton} type="button" onClick={props.onCancel}>
                    Отмена
                </button>
                <button class={styles.confirmButton} type="button" onClick={props.onConfirm}>
                    {props.confirmLabel}
                </button>
            </div>
        </Modal>
    );
}