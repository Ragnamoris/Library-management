import { For, Show, createSignal } from "solid-js";
import type { JSX } from "solid-js";
import { FiCheck, FiEdit2, FiPlus, FiTrash2 } from "solid-icons/fi";

import { mockPublishers } from "../data/mocks";
import type { Publisher } from "../data/types";
import { formatDate } from "../lib/formatDate";
import { ConfirmDialog } from "../ui/modal/ConfirmDialog";
import { Modal } from "../ui/modal/Modal";

import styles from "../ui/pages.module.css";

interface PublisherFormErrors {
    name?: string;
    address?: string;
}

export function PublishersPage(): JSX.Element {
    const [publishers, setPublishers] = createSignal<Publisher[]>(mockPublishers);
    const [publisherName, setPublisherName] = createSignal("");
    const [publisherAddress, setPublisherAddress] = createSignal("");
    const [errors, setErrors] = createSignal<PublisherFormErrors>({});
    const [feedback, setFeedback] = createSignal("");

    const [editingPublisher, setEditingPublisher] = createSignal<Publisher | null>(null);
    const [deletingPublisher, setDeletingPublisher] = createSignal<Publisher | null>(null);
    const [editName, setEditName] = createSignal("");
    const [editAddress, setEditAddress] = createSignal("");
    const [editErrors, setEditErrors] = createSignal<PublisherFormErrors>({});

    const [selectedIds, setSelectedIds] = createSignal<string[]>([]);
    const [bulkDeleteOpen, setBulkDeleteOpen] = createSignal(false);

    function allSelected(): boolean {
        const ids = selectedIds();
        return (
            publishers().length > 0 &&
            publishers().every((publisher) => ids.includes(publisher.id))
        );
    }

    function toggleAll(): void {
        if (allSelected()) {
            setSelectedIds([]);
        } else {
            setSelectedIds(publishers().map((publisher) => publisher.id));
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

    function addPublisher(event: SubmitEvent): void {
        event.preventDefault();

        const name = publisherName().trim();
        const address = publisherAddress().trim();

        const nextErrors: PublisherFormErrors = {};

        if (name.length < 2 || name.length > 256) {
            nextErrors.name = "Наименование должно содержать от 2 до 256 символов";
        }

        if (address.length < 3 || address.length > 256) {
            nextErrors.address = "Адрес должен содержать от 3 до 256 символов";
        }

        setErrors(nextErrors);

        if (nextErrors.name || nextErrors.address) {
            return;
        }

        const newPublisher: Publisher = {
            id: crypto.randomUUID(),
            name,
            address,
            created_at: new Date().toISOString(),
            updated_at: null,
            deleted_at: null,
        };

        setPublishers((currentPublishers) => [...currentPublishers, newPublisher]);

        setPublisherName("");
        setPublisherAddress("");
        setFeedback("Издательство добавлено в список");
    }

    function openEdit(publisher: Publisher): void {
        setEditingPublisher(publisher);
        setEditName(publisher.name);
        setEditAddress(publisher.address);
        setEditErrors({});
        setFeedback("");
    }

    function closeEdit(): void {
        setEditingPublisher(null);
    }

    function saveEdit(event: SubmitEvent): void {
        event.preventDefault();

        const current = editingPublisher();

        if (!current) {
            return;
        }

        const name = editName().trim();
        const address = editAddress().trim();

        const nextErrors: PublisherFormErrors = {};

        if (name.length < 2 || name.length > 256) {
            nextErrors.name = "Наименование должно содержать от 2 до 256 символов";
        }

        if (address.length < 3 || address.length > 256) {
            nextErrors.address = "Адрес должен содержать от 3 до 256 символов";
        }

        setEditErrors(nextErrors);

        if (nextErrors.name || nextErrors.address) {
            return;
        }

        setPublishers((currentPublishers) =>
            currentPublishers.map((publisher) =>
                publisher.id === current.id
                    ? { ...publisher, name, address, updated_at: new Date().toISOString() }
                    : publisher,
            ),
        );

        setEditingPublisher(null);
        setFeedback("Изменения сохранены");
    }

    function confirmDelete(): void {
        const current = deletingPublisher();

        if (!current) {
            return;
        }

        setPublishers((currentPublishers) =>
            currentPublishers.filter((publisher) => publisher.id !== current.id),
        );
        setSelectedIds((currentIds) => currentIds.filter((id) => id !== current.id));

        setDeletingPublisher(null);
        setFeedback("Запись удалена");
    }

    function confirmBulkDelete(): void {
        const ids = selectedIds();

        if (ids.length === 0) {
            setBulkDeleteOpen(false);
            return;
        }

        setPublishers((currentPublishers) =>
            currentPublishers.filter((publisher) => !ids.includes(publisher.id)),
        );

        setSelectedIds([]);
        setBulkDeleteOpen(false);
        setFeedback(`Удалено записей: ${ids.length}`);
    }

    return (
        <div class={styles.page}>
            <h1 class={styles.title} id="publishers-title">Издательства</h1>

            <form
                class={styles.form}
                aria-labelledby="publishers-title"
                onSubmit={addPublisher}
                noValidate
            >
                <div class={styles.field}>
                    <label for="publisher-name">Наименование</label>
                    <input
                        id="publisher-name"
                        name="name"
                        type="text"
                        placeholder="Например: Просвещение"
                        required
                        minLength={2}
                        maxLength={256}
                        value={publisherName()}
                        onInput={(event) => {
                            setPublisherName(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().name ? true : undefined}
                        aria-describedby={errors().name ? "publisher-name-error" : undefined}
                    />
                    <Show when={errors().name}>
                        <p class={styles.error} id="publisher-name-error" role="alert">
                            {errors().name}
                        </p>
                    </Show>
                </div>

                <div class={styles.field}>
                    <label for="publisher-address">Адрес</label>
                    <input
                        id="publisher-address"
                        name="address"
                        type="text"
                        placeholder="Например: г. Москва, ул. Кирова, д. 12"
                        required
                        minLength={3}
                        maxLength={256}
                        value={publisherAddress()}
                        onInput={(event) => {
                            setPublisherAddress(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().address ? true : undefined}
                        aria-describedby={errors().address ? "publisher-address-error" : undefined}
                    />
                    <Show when={errors().address}>
                        <p class={styles.error} id="publisher-address-error" role="alert">
                            {errors().address}
                        </p>
                    </Show>
                </div>

                <div class={styles.formActions}>
                    <button class={styles.submitButton} type="submit">
                        <FiPlus size={16} aria-hidden="true" />
                        Добавить издательство
                    </button>
                </div>

                <Show when={feedback()}>
                    <p class={styles.feedback} role="status">
                        {feedback()}
                    </p>
                </Show>
            </form>

            <p class={styles.counter}>Всего записей: {publishers().length}</p>

            <Show when={selectedIds().length > 0}>
                <div class={styles.bulkBar}>
                    <span class={styles.bulkText}>Выбрано записей: {selectedIds().length}</span>
                    <button
                        class={styles.bulkDeleteButton}
                        type="button"
                        onClick={() => setBulkDeleteOpen(true)}
                    >
                        <FiTrash2 size={16} aria-hidden="true" />
                        Удалить выбранные
                    </button>
                </div>
            </Show>

            <Show
                when={publishers().length > 0}
                fallback={<p class={styles.empty}>Издательств пока нет</p>}
            >
                <div class={styles.tableScroll}>
                    <table
                        class={styles.table}
                        aria-labelledby="publishers-title"
                        aria-label="Таблица издательств"
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
                                <th scope="col">Наименование</th>
                                <th scope="col">Адрес</th>
                                <th scope="col">Создано (UTC)</th>
                                <th scope="col">Обновлено (UTC)</th>
                                <th scope="col">Удалено (UTC)</th>
                                <th scope="col">Действия</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For each={publishers()}>
                                {(publisher) => (
                                    <tr
                                        onDblClick={(event) =>
                                            handleRowDoubleClick(event, publisher.id)
                                        }
                                    >
                                        <td class={styles.checkboxColumn}>
                                            <input
                                                type="checkbox"
                                                checked={selectedIds().includes(publisher.id)}
                                                onChange={() => toggleRow(publisher.id)}
                                                aria-label={`Выбрать издательство ${publisher.name}`}
                                            />
                                        </td>
                                        <td>{publisher.name}</td>
                                        <td>{publisher.address}</td>
                                        <td>{formatDate(publisher.created_at)}</td>
                                        <td>{formatDate(publisher.updated_at)}</td>
                                        <td>{formatDate(publisher.deleted_at)}</td>
                                        <td>
                                            <div class={styles.rowActions}>
                                                <button
                                                    class={styles.iconButton}
                                                    type="button"
                                                    onClick={() => openEdit(publisher)}
                                                    aria-label="Редактировать издательство"
                                                >
                                                    <FiEdit2 size={16} aria-hidden="true" />
                                                </button>
                                                <button
                                                    class={styles.iconButton}
                                                    type="button"
                                                    onClick={() => setDeletingPublisher(publisher)}
                                                    aria-label="Удалить издательство"
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

            <Show when={editingPublisher()}>
                <Modal title="Редактировать издательство" onClose={closeEdit}>
                    <form class={styles.form} onSubmit={saveEdit} noValidate>
                        <div class={styles.field}>
                            <label for="publisher-edit-name">Наименование</label>
                            <input
                                id="publisher-edit-name"
                                name="name"
                                type="text"
                                required
                                minLength={2}
                                maxLength={256}
                                value={editName()}
                                onInput={(event) => setEditName(event.currentTarget.value)}
                                aria-invalid={editErrors().name ? true : undefined}
                                aria-describedby={
                                    editErrors().name ? "publisher-edit-name-error" : undefined
                                }
                            />
                            <Show when={editErrors().name}>
                                <p class={styles.error} id="publisher-edit-name-error" role="alert">
                                    {editErrors().name}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="publisher-edit-address">Адрес</label>
                            <input
                                id="publisher-edit-address"
                                name="address"
                                type="text"
                                required
                                minLength={3}
                                maxLength={256}
                                value={editAddress()}
                                onInput={(event) => setEditAddress(event.currentTarget.value)}
                                aria-invalid={editErrors().address ? true : undefined}
                                aria-describedby={
                                    editErrors().address
                                        ? "publisher-edit-address-error"
                                        : undefined
                                }
                            />
                            <Show when={editErrors().address}>
                                <p
                                    class={styles.error}
                                    id="publisher-edit-address-error"
                                    role="alert"
                                >
                                    {editErrors().address}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.formActions}>
                            <button class={styles.submitButton} type="submit">
                                <FiCheck size={16} aria-hidden="true" />
                                Сохранить изменения
                            </button>
                        </div>
                    </form>
                </Modal>
            </Show>

            <Show when={deletingPublisher()}>
                {(publisher) => (
                    <ConfirmDialog
                        title="Удаление издательства"
                        message={`Удалить эту запись? «${publisher().name}»`}
                        confirmLabel="Удалить"
                        onConfirm={confirmDelete}
                        onCancel={() => setDeletingPublisher(null)}
                    />
                )}
            </Show>

            <Show when={bulkDeleteOpen()}>
                <ConfirmDialog
                    title="Массовое удаление"
                    message={`Удалить выбранные записи (${selectedIds().length})?`}
                    confirmLabel="Удалить"
                    onConfirm={confirmBulkDelete}
                    onCancel={() => setBulkDeleteOpen(false)}
                />
            </Show>
        </div>
    );
}