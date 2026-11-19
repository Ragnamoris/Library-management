import { For, Show, createSignal } from "solid-js";
import type { JSX } from "solid-js";
import { FiCheck, FiEdit2, FiPlus, FiTrash2 } from "solid-icons/fi";
import { mockReaders } from "../data/mocks";
import type { Reader } from "../data/types";
import { formatDate } from "../lib/formatDate";
import { ConfirmDialog } from "../ui/modal/ConfirmDialog";
import { Modal } from "../ui/modal/Modal";
import styles from "../ui/pages.module.css";

interface ReaderFormErrors {
    full_name?: string;
    phone?: string;
    card_number?: string;
}

const cancelButtonStyle: JSX.CSSProperties = {
    padding: "10px 16px",
    "border-radius": "8px",
    border: "1px solid var(--color-accent)",
    "background-color": "transparent",
    color: "var(--color-text)",
    font: "inherit",
    cursor: "pointer",
};

export function ReadersPage(): JSX.Element {
    const [readers, setReaders] = createSignal<Reader[]>(mockReaders);
    const [fullName, setFullName] = createSignal("");
    const [phone, setPhone] = createSignal("");
    const [cardNumber, setCardNumber] = createSignal("");
    const [errors, setErrors] = createSignal<ReaderFormErrors>({});
    const [feedback, setFeedback] = createSignal("");
    const [editingReader, setEditingReader] = createSignal<Reader | null>(null);
    const [deletingReader, setDeletingReader] = createSignal<Reader | null>(null);
    const [editFullName, setEditFullName] = createSignal("");
    const [editPhone, setEditPhone] = createSignal("");
    const [editCardNumber, setEditCardNumber] = createSignal("");
    const [editErrors, setEditErrors] = createSignal<ReaderFormErrors>({});
    const [selectedIds, setSelectedIds] = createSignal<string[]>([]);
    const [readersToDelete, setReadersToDelete] = createSignal<Reader[]>([]);

    function allSelected(): boolean {
        const ids = selectedIds();
        return readers().length > 0 && readers().every((reader) => ids.includes(reader.id));
    }

    function toggleAll(): void {
        if (allSelected()) {
            setSelectedIds([]);
        } else {
            setSelectedIds(readers().map((reader) => reader.id));
        }
    }

    function toggleRow(id: string): void {
        setSelectedIds((current) =>
            current.includes(id)
                ? current.filter((item) => item !== id)
                : [...current, id],
        );
    }

    function handleRowDoubleClick(event: MouseEvent, id: string): void {
        const target = event.target;
        if (target instanceof Element && target.closest("input, button, select, a")) {
            return;
        }
        toggleRow(id);
    }

    function rowStyle(id: string): JSX.CSSProperties | undefined {
        return selectedIds().includes(id)
            ? {
                  "background-color": "rgba(233, 69, 96, 0.12)",
                  "box-shadow": "inset 3px 0 0 var(--color-highlight)",
              }
            : undefined;
    }

    function addReader(event: SubmitEvent): void {
        event.preventDefault();
        const nextFullName = fullName().trim();
        const nextPhone = phone().trim();
        const nextCardNumber = cardNumber().trim();
        const nextErrors: ReaderFormErrors = {};
        if (nextFullName.length < 2 || nextFullName.length > 128) {
            nextErrors.full_name = "ФИО должно содержать от 2 до 128 символов";
        }
        if (nextPhone.length < 6 || nextPhone.length > 32) {
            nextErrors.phone = "Телефон должен содержать от 6 до 32 символов";
        } else if (readers().some((reader) => reader.phone === nextPhone)) {
            nextErrors.phone = "Читатель с таким телефоном уже есть";
        }
        if (nextCardNumber.length < 3 || nextCardNumber.length > 32) {
            nextErrors.card_number = "Номер билета должен содержать от 3 до 32 символов";
        } else if (readers().some((reader) => reader.card_number === nextCardNumber)) {
            nextErrors.card_number = "Читатель с таким номером билета уже есть";
        }
        setErrors(nextErrors);
        if (nextErrors.full_name || nextErrors.phone || nextErrors.card_number) {
            return;
        }
        const newReader: Reader = {
            id: crypto.randomUUID(),
            full_name: nextFullName,
            phone: nextPhone,
            card_number: nextCardNumber,
            created_at: new Date().toISOString(),
            updated_at: null,
            deleted_at: null,
        };
        setReaders((currentReaders) => [...currentReaders, newReader]);
        setFullName("");
        setPhone("");
        setCardNumber("");
        setFeedback("Читатель добавлен в список");
    }

    function openEdit(reader: Reader): void {
        setEditingReader(reader);
        setEditFullName(reader.full_name);
        setEditPhone(reader.phone);
        setEditCardNumber(reader.card_number);
        setEditErrors({});
        setFeedback("");
    }

    function closeEdit(): void {
        setEditingReader(null);
        setEditFullName("");
        setEditPhone("");
        setEditCardNumber("");
        setEditErrors({});
    }

    function saveEdit(event: SubmitEvent): void {
        event.preventDefault();
        const target = editingReader();
        if (!target) {
            return;
        }
        const nextFullName = editFullName().trim();
        const nextPhone = editPhone().trim();
        const nextCardNumber = editCardNumber().trim();
        const nextErrors: ReaderFormErrors = {};
        if (nextFullName.length < 2 || nextFullName.length > 128) {
            nextErrors.full_name = "ФИО должно содержать от 2 до 128 символов";
        }
        if (nextPhone.length < 6 || nextPhone.length > 32) {
            nextErrors.phone = "Телефон должен содержать от 6 до 32 символов";
        } else if (
            readers().some((reader) => reader.id !== target.id && reader.phone === nextPhone)
        ) {
            nextErrors.phone = "Читатель с таким телефоном уже есть";
        }
        if (nextCardNumber.length < 3 || nextCardNumber.length > 32) {
            nextErrors.card_number = "Номер билета должен содержать от 3 до 32 символов";
        } else if (
            readers().some(
                (reader) => reader.id !== target.id && reader.card_number === nextCardNumber,
            )
        ) {
            nextErrors.card_number = "Читатель с таким номером билета уже есть";
        }
        setEditErrors(nextErrors);
        if (nextErrors.full_name || nextErrors.phone || nextErrors.card_number) {
            return;
        }
        setReaders((currentReaders) =>
            currentReaders.map((reader) =>
                reader.id === target.id
                    ? {
                          ...reader,
                          full_name: nextFullName,
                          phone: nextPhone,
                          card_number: nextCardNumber,
                          updated_at: new Date().toISOString(),
                      }
                    : reader,
            ),
        );
        setFeedback("Изменения сохранены");
        closeEdit();
    }

    function confirmDelete(): void {
        const target = deletingReader();
        if (!target) {
            return;
        }
        setReaders((currentReaders) => currentReaders.filter((reader) => reader.id !== target.id));
        setSelectedIds((currentIds) => currentIds.filter((id) => id !== target.id));
        setDeletingReader(null);
        setFeedback("Запись удалена");
    }

    function requestBulkDelete(): void {
        const ids = selectedIds();
        const chosen = readers().filter((reader) => ids.includes(reader.id));
        if (chosen.length === 0) {
            return;
        }
        setReadersToDelete(chosen);
    }

    function cancelBulkDelete(): void {
        setReadersToDelete([]);
    }

    function confirmBulkDelete(): void {
        const chosen = readersToDelete();
        if (chosen.length === 0) {
            return;
        }
        const idsToDelete = chosen.map((reader) => reader.id);
        setReaders((currentReaders) =>
            currentReaders.filter((reader) => !idsToDelete.includes(reader.id)),
        );
        setSelectedIds((currentIds) => currentIds.filter((id) => !idsToDelete.includes(id)));
        setReadersToDelete([]);
        setFeedback(`Удалено записей: ${idsToDelete.length}`);
    }

    return (
        <div class={styles.page}>
            <h1 class={styles.title} id="readers-title">Читатели</h1>

            <form
                class={styles.form}
                aria-labelledby="readers-title"
                onSubmit={addReader}
                noValidate
            >
                <div class={styles.field}>
                    <label for="reader-full-name">ФИО</label>
                    <input
                        id="reader-full-name"
                        name="full_name"
                        type="text"
                        placeholder="Например: Иванов Иван Иванович"
                        required
                        minLength={2}
                        maxLength={128}
                        value={fullName()}
                        onInput={(event) => {
                            setFullName(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().full_name ? true : undefined}
                        aria-describedby={errors().full_name ? "reader-full-name-error" : undefined}
                    />
                    <Show when={errors().full_name}>
                        <p class={styles.error} id="reader-full-name-error" role="alert">
                            {errors().full_name}
                        </p>
                    </Show>
                </div>

                <div class={styles.field}>
                    <label for="reader-phone">Телефон</label>
                    <input
                        id="reader-phone"
                        name="phone"
                        type="text"
                        placeholder="Например: +7 (900) 000-00-00"
                        required
                        minLength={6}
                        maxLength={32}
                        value={phone()}
                        onInput={(event) => {
                            setPhone(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().phone ? true : undefined}
                        aria-describedby={errors().phone ? "reader-phone-error" : undefined}
                    />
                    <Show when={errors().phone}>
                        <p class={styles.error} id="reader-phone-error" role="alert">
                            {errors().phone}
                        </p>
                    </Show>
                </div>

                <div class={styles.field}>
                    <label for="reader-card">Номер читательского билета</label>
                    <input
                        id="reader-card"
                        name="card_number"
                        type="text"
                        placeholder="Например: ЧБ-0004"
                        required
                        minLength={3}
                        maxLength={32}
                        value={cardNumber()}
                        onInput={(event) => {
                            setCardNumber(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().card_number ? true : undefined}
                        aria-describedby={errors().card_number ? "reader-card-error" : undefined}
                    />
                    <Show when={errors().card_number}>
                        <p class={styles.error} id="reader-card-error" role="alert">
                            {errors().card_number}
                        </p>
                    </Show>
                </div>

                <div class={styles.formActions}>
                    <button class={styles.submitButton} type="submit">
                        <FiPlus size={16} aria-hidden="true" />
                        Добавить читателя
                    </button>
                </div>

                <Show when={feedback()}>
                    <p class={styles.feedback} role="status">
                        {feedback()}
                    </p>
                </Show>
            </form>

            <p class={styles.counter}>Всего записей: {readers().length}</p>

            <div class={styles.bulkBar}>
                <span class={styles.bulkText}>Выбрано записей: {selectedIds().length}</span>
                <button
                    class={styles.bulkDeleteButton}
                    type="button"
                    onClick={requestBulkDelete}
                    disabled={selectedIds().length === 0}
                    style={{
                        opacity: selectedIds().length === 0 ? "0.45" : "1",
                        cursor: selectedIds().length === 0 ? "not-allowed" : "pointer",
                    }}
                >
                    <FiTrash2 size={16} aria-hidden="true" />
                    Удалить выбранные
                </button>
            </div>

            <Show
                when={readers().length > 0}
                fallback={<p class={styles.empty}>Читателей пока нет</p>}
            >
                <div class={styles.tableScroll}>
                    <table
                        class={styles.table}
                        aria-labelledby="readers-title"
                        aria-label="Таблица читателей"
                    >
                        <thead>
                            <tr>
                                <th scope="col" class={styles.checkboxColumn}>
                                    <input
                                        type="checkbox"
                                        checked={allSelected()}
                                        onChange={toggleAll}
                                        aria-label="Выбрать все записи"
                                    />
                                </th>
                                <th scope="col">ФИО</th>
                                <th scope="col">Телефон</th>
                                <th scope="col">Номер билета</th>
                                <th scope="col">Создано (UTC)</th>
                                <th scope="col">Обновлено (UTC)</th>
                                <th scope="col">Удалено (UTC)</th>
                                <th scope="col">Действия</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For each={readers()}>
                                {(reader) => (
                                    <tr
                                        style={rowStyle(reader.id)}
                                        onDblClick={(event) => handleRowDoubleClick(event, reader.id)}
                                    >
                                        <td class={styles.checkboxColumn}>
                                            <input
                                                type="checkbox"
                                                checked={selectedIds().includes(reader.id)}
                                                onChange={() => toggleRow(reader.id)}
                                                aria-label={`Выбрать читателя ${reader.full_name}`}
                                            />
                                        </td>
                                        <td>{reader.full_name}</td>
                                        <td>{reader.phone}</td>
                                        <td>{reader.card_number}</td>
                                        <td>{formatDate(reader.created_at)}</td>
                                        <td>{formatDate(reader.updated_at)}</td>
                                        <td>{formatDate(reader.deleted_at)}</td>
                                        <td>
                                            <div class={styles.rowActions}>
                                                <button
                                                    id={`reader-edit-button-${reader.id}`}
                                                    class={styles.iconButton}
                                                    type="button"
                                                    onClick={() => openEdit(reader)}
                                                    aria-label="Редактировать читателя"
                                                >
                                                    <FiEdit2 size={16} aria-hidden="true" />
                                                </button>
                                                <button
                                                    class={styles.iconButton}
                                                    type="button"
                                                    onClick={() => setDeletingReader(reader)}
                                                    aria-label="Удалить читателя"
                                                >
                                                    <FiTrash2 size={16} aria-hidden="true" />
                                                </button>
                                            </div>
                                        </td>
                                    </tr>
                                )}
                            </For>
                        </tbody>
                    </table>
                </div>
            </Show>

            <Show when={editingReader()}>
                <Modal title="Редактировать читателя" onClose={closeEdit}>
                    <form class={styles.form} onSubmit={saveEdit} noValidate>
                        <div class={styles.field}>
                            <label for="reader-edit-full-name">ФИО</label>
                            <input
                                id="reader-edit-full-name"
                                name="full_name"
                                type="text"
                                required
                                autofocus
                                minLength={2}
                                maxLength={128}
                                value={editFullName()}
                                onInput={(event) => setEditFullName(event.currentTarget.value)}
                                aria-invalid={editErrors().full_name ? true : undefined}
                                aria-describedby={
                                    editErrors().full_name
                                        ? "reader-edit-full-name-error"
                                        : undefined
                                }
                            />
                            <Show when={editErrors().full_name}>
                                <p
                                    class={styles.error}
                                    id="reader-edit-full-name-error"
                                    role="alert"
                                >
                                    {editErrors().full_name}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="reader-edit-phone">Телефон</label>
                            <input
                                id="reader-edit-phone"
                                name="phone"
                                type="text"
                                required
                                minLength={6}
                                maxLength={32}
                                value={editPhone()}
                                onInput={(event) => setEditPhone(event.currentTarget.value)}
                                aria-invalid={editErrors().phone ? true : undefined}
                                aria-describedby={
                                    editErrors().phone ? "reader-edit-phone-error" : undefined
                                }
                            />
                            <Show when={editErrors().phone}>
                                <p class={styles.error} id="reader-edit-phone-error" role="alert">
                                    {editErrors().phone}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="reader-edit-card">Номер читательского билета</label>
                            <input
                                id="reader-edit-card"
                                name="card_number"
                                type="text"
                                required
                                minLength={3}
                                maxLength={32}
                                value={editCardNumber()}
                                onInput={(event) => setEditCardNumber(event.currentTarget.value)}
                                aria-invalid={editErrors().card_number ? true : undefined}
                                aria-describedby={
                                    editErrors().card_number
                                        ? "reader-edit-card-error"
                                        : undefined
                                }
                            />
                            <Show when={editErrors().card_number}>
                                <p class={styles.error} id="reader-edit-card-error" role="alert">
                                    {editErrors().card_number}
                                </p>
                            </Show>
                        </div>

                        <div
    class={styles.formActions}
    style={{
        display: "flex",
        "justify-content": "flex-end",
        "flex-wrap": "wrap",
        gap: "10px",
    }}
>
    <button class={styles.submitButton} type="submit">
        <FiCheck size={16} aria-hidden="true" />
        Сохранить изменения
    </button>
    <button
        type="button"
        style={cancelButtonStyle}
        onClick={closeEdit}
        aria-label="Отменить редактирование"
    >
        Отмена
    </button>
</div>
                    </form>
                </Modal>
            </Show>

            <Show when={deletingReader()}>
                {(reader) => (
                    <ConfirmDialog
                        title="Удаление читателя"
                        message={`Удалить эту запись? «${reader().full_name}»`}
                        confirmLabel="Удалить"
                        onConfirm={confirmDelete}
                        onCancel={() => setDeletingReader(null)}
                    />
                )}
            </Show>

            <Show when={readersToDelete().length > 0}>
                <ConfirmDialog
                    title="Массовое удаление"
                    message={`Удалить выбранные записи (${readersToDelete().length}): ${readersToDelete()
                        .map((reader) => `«${reader.full_name}»`)
                        .join(", ")}?`}
                    confirmLabel="Удалить"
                    onConfirm={confirmBulkDelete}
                    onCancel={cancelBulkDelete}
                />
            </Show>
        </div>
    );
}