import { For, Show, createSignal } from "solid-js";
import type { JSX } from "solid-js";
import { FiCheck, FiEdit2, FiPlus, FiTrash2 } from "solid-icons/fi";
import { mockBooks, mockPublishers } from "../data/mocks";
import type { Book } from "../data/types";
import { formatDate } from "../lib/formatDate";
import { ConfirmDialog } from "../ui/modal/ConfirmDialog";
import { Modal } from "../ui/modal/Modal";
import styles from "../ui/pages.module.css";

interface BookFormErrors {
    publisher_id?: string;
    title?: string;
    author?: string;
    year?: string;
    inventory_number?: string;
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

export function BooksPage(): JSX.Element {
    const [books, setBooks] = createSignal<Book[]>(mockBooks);
    const [publisherId, setPublisherId] = createSignal("");
    const [title, setTitle] = createSignal("");
    const [author, setAuthor] = createSignal("");
    const [year, setYear] = createSignal("");
    const [inventoryNumber, setInventoryNumber] = createSignal("");
    const [errors, setErrors] = createSignal<BookFormErrors>({});
    const [feedback, setFeedback] = createSignal("");
    const [editingBook, setEditingBook] = createSignal<Book | null>(null);
    const [deletingBook, setDeletingBook] = createSignal<Book | null>(null);
    const [editPublisherId, setEditPublisherId] = createSignal("");
    const [editTitle, setEditTitle] = createSignal("");
    const [editAuthor, setEditAuthor] = createSignal("");
    const [editYear, setEditYear] = createSignal("");
    const [editInventory, setEditInventory] = createSignal("");
    const [editErrors, setEditErrors] = createSignal<BookFormErrors>({});
    const [selectedIds, setSelectedIds] = createSignal<string[]>([]);
    const [booksToDelete, setBooksToDelete] = createSignal<Book[]>([]);

    function publisherName(id: string): string {
        const publisher = mockPublishers.find((item) => item.id === id);
        return publisher ? publisher.name : "Неизвестное издательство";
    }

    function allSelected(): boolean {
        const ids = selectedIds();
        return books().length > 0 && books().every((book) => ids.includes(book.id));
    }

    function toggleAll(): void {
        if (allSelected()) {
            setSelectedIds([]);
        } else {
            setSelectedIds(books().map((book) => book.id));
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

    function addBook(event: SubmitEvent): void {
        event.preventDefault();
        const nextPublisherId = publisherId();
        const nextTitle = title().trim();
        const nextAuthor = author().trim();
        const nextYear = Number(year());
        const nextInventory = inventoryNumber().trim();
        const nextErrors: BookFormErrors = {};
        if (nextPublisherId === "") {
            nextErrors.publisher_id = "Выберите издательство из списка";
        }
        if (nextTitle.length < 2 || nextTitle.length > 256) {
            nextErrors.title = "Название должно содержать от 2 до 256 символов";
        }
        if (nextAuthor.length < 2 || nextAuthor.length > 256) {
            nextErrors.author = "Поле «Автор» должно содержать от 2 до 256 символов";
        }
        if (!Number.isInteger(nextYear) || nextYear < 1450 || nextYear > 2026) {
            nextErrors.year = "Укажите целое число от 1450 до 2026";
        }
        if (nextInventory.length < 3 || nextInventory.length > 64) {
            nextErrors.inventory_number =
                "Инвентарный номер должен содержать от 3 до 64 символов";
        } else if (books().some((book) => book.inventory_number === nextInventory)) {
            nextErrors.inventory_number = "Книга с таким инвентарным номером уже есть";
        }
        setErrors(nextErrors);
        if (
            nextErrors.publisher_id ||
            nextErrors.title ||
            nextErrors.author ||
            nextErrors.year ||
            nextErrors.inventory_number
        ) {
            return;
        }
        const newBook: Book = {
            id: crypto.randomUUID(),
            publisher_id: nextPublisherId,
            title: nextTitle,
            author: nextAuthor,
            year: nextYear,
            inventory_number: nextInventory,
            created_at: new Date().toISOString(),
            updated_at: null,
            deleted_at: null,
        };
        setBooks((currentBooks) => [...currentBooks, newBook]);
        setPublisherId("");
        setTitle("");
        setAuthor("");
        setYear("");
        setInventoryNumber("");
        setFeedback("Книга добавлена в список");
    }

    function openEdit(book: Book): void {
        setEditingBook(book);
        setEditPublisherId(book.publisher_id);
        setEditTitle(book.title);
        setEditAuthor(book.author);
        setEditYear(String(book.year));
        setEditInventory(book.inventory_number);
        setEditErrors({});
        setFeedback("");
    }

    function closeEdit(): void {
        setEditingBook(null);
        setEditPublisherId("");
        setEditTitle("");
        setEditAuthor("");
        setEditYear("");
        setEditInventory("");
        setEditErrors({});
    }

    function saveEdit(event: SubmitEvent): void {
        event.preventDefault();
        const current = editingBook();
        if (!current) {
            return;
        }
        const nextPublisherId = editPublisherId();
        const nextTitle = editTitle().trim();
        const nextAuthor = editAuthor().trim();
        const nextYear = Number(editYear());
        const nextInventory = editInventory().trim();
        const nextErrors: BookFormErrors = {};
        if (nextPublisherId === "") {
            nextErrors.publisher_id = "Выберите издательство из списка";
        }
        if (nextTitle.length < 2 || nextTitle.length > 256) {
            nextErrors.title = "Название должно содержать от 2 до 256 символов";
        }
        if (nextAuthor.length < 2 || nextAuthor.length > 256) {
            nextErrors.author = "Поле «Автор» должно содержать от 2 до 256 символов";
        }
        if (!Number.isInteger(nextYear) || nextYear < 1450 || nextYear > 2026) {
            nextErrors.year = "Укажите целое число от 1450 до 2026";
        }
        if (nextInventory.length < 3 || nextInventory.length > 64) {
            nextErrors.inventory_number =
                "Инвентарный номер должен содержать от 3 до 64 символов";
        } else if (
            books().some(
                (book) => book.id !== current.id && book.inventory_number === nextInventory,
            )
        ) {
            nextErrors.inventory_number = "Книга с таким инвентарным номером уже есть";
        }
        setEditErrors(nextErrors);
        if (
            nextErrors.publisher_id ||
            nextErrors.title ||
            nextErrors.author ||
            nextErrors.year ||
            nextErrors.inventory_number
        ) {
            return;
        }
        setBooks((currentBooks) =>
            currentBooks.map((book) =>
                book.id === current.id
                    ? {
                          ...book,
                          publisher_id: nextPublisherId,
                          title: nextTitle,
                          author: nextAuthor,
                          year: nextYear,
                          inventory_number: nextInventory,
                          updated_at: new Date().toISOString(),
                      }
                    : book,
            ),
        );
        setFeedback("Изменения сохранены");
        closeEdit();
    }

    function confirmDelete(): void {
        const current = deletingBook();
        if (!current) {
            return;
        }
        setBooks((currentBooks) => currentBooks.filter((book) => book.id !== current.id));
        setSelectedIds((currentIds) => currentIds.filter((id) => id !== current.id));
        setDeletingBook(null);
        setFeedback("Запись удалена");
    }

    function requestBulkDelete(): void {
        const ids = selectedIds();
        const chosen = books().filter((book) => ids.includes(book.id));
        if (chosen.length === 0) {
            return;
        }
        setBooksToDelete(chosen);
    }

    function cancelBulkDelete(): void {
        setBooksToDelete([]);
    }

    function confirmBulkDelete(): void {
        const chosen = booksToDelete();
        if (chosen.length === 0) {
            return;
        }
        const idsToDelete = chosen.map((book) => book.id);
        setBooks((currentBooks) =>
            currentBooks.filter((book) => !idsToDelete.includes(book.id)),
        );
        setSelectedIds((currentIds) => currentIds.filter((id) => !idsToDelete.includes(id)));
        setBooksToDelete([]);
        setFeedback(`Удалено записей: ${idsToDelete.length}`);
    }

    return (
        <div class={styles.page}>
            <h1 class={styles.title} id="books-title">Книги</h1>

            <form class={styles.form} aria-labelledby="books-title" onSubmit={addBook} noValidate>
                <div class={styles.field}>
                    <label for="book-publisher">Издательство</label>
                    <select
                        id="book-publisher"
                        name="publisher_id"
                        value={publisherId()}
                        onInput={(event) => {
                            setPublisherId(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().publisher_id ? true : undefined}
                        aria-describedby={errors().publisher_id ? "book-publisher-error" : undefined}
                    >
                        <option value="">— выберите издательство —</option>
                        <For each={mockPublishers}>
                            {(publisher) => <option value={publisher.id}>{publisher.name}</option>}
                        </For>
                    </select>
                    <Show when={errors().publisher_id}>
                        <p class={styles.error} id="book-publisher-error" role="alert">
                            {errors().publisher_id}
                        </p>
                    </Show>
                </div>

                <div class={styles.field}>
                    <label for="book-title">Название</label>
                    <input
                        id="book-title"
                        name="title"
                        type="text"
                        placeholder="Например: Мастер и Маргарита"
                        required
                        minLength={2}
                        maxLength={256}
                        value={title()}
                        onInput={(event) => {
                            setTitle(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().title ? true : undefined}
                        aria-describedby={errors().title ? "book-title-error" : undefined}
                    />
                    <Show when={errors().title}>
                        <p class={styles.error} id="book-title-error" role="alert">
                            {errors().title}
                        </p>
                    </Show>
                </div>

                <div class={styles.field}>
                    <label for="book-author">Автор</label>
                    <input
                        id="book-author"
                        name="author"
                        type="text"
                        placeholder="Например: М. А. Булгаков"
                        required
                        minLength={2}
                        maxLength={256}
                        value={author()}
                        onInput={(event) => {
                            setAuthor(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().author ? true : undefined}
                        aria-describedby={errors().author ? "book-author-error" : undefined}
                    />
                    <Show when={errors().author}>
                        <p class={styles.error} id="book-author-error" role="alert">
                            {errors().author}
                        </p>
                    </Show>
                </div>

                <div class={styles.field}>
                    <label for="book-year">Год издания</label>
                    <input
                        id="book-year"
                        name="year"
                        type="number"
                        placeholder="Например: 2019"
                        required
                        min={1450}
                        max={2026}
                        step={1}
                        value={year()}
                        onInput={(event) => {
                            setYear(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().year ? true : undefined}
                        aria-describedby={errors().year ? "book-year-error" : undefined}
                    />
                    <Show when={errors().year}>
                        <p class={styles.error} id="book-year-error" role="alert">
                            {errors().year}
                        </p>
                    </Show>
                </div>

                <div class={styles.field}>
                    <label for="book-inventory">Инвентарный номер</label>
                    <input
                        id="book-inventory"
                        name="inventory_number"
                        type="text"
                        placeholder="Например: ИНВ-0004"
                        required
                        minLength={3}
                        maxLength={64}
                        value={inventoryNumber()}
                        onInput={(event) => {
                            setInventoryNumber(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().inventory_number ? true : undefined}
                        aria-describedby={
                            errors().inventory_number ? "book-inventory-error" : undefined
                        }
                    />
                    <Show when={errors().inventory_number}>
                        <p class={styles.error} id="book-inventory-error" role="alert">
                            {errors().inventory_number}
                        </p>
                    </Show>
                </div>

                <div class={styles.formActions}>
                    <button class={styles.submitButton} type="submit">
                        <FiPlus size={16} aria-hidden="true" />
                        Добавить книгу
                    </button>
                </div>

                <Show when={feedback()}>
                    <p class={styles.feedback} role="status">
                        {feedback()}
                    </p>
                </Show>
            </form>

            <p class={styles.counter}>Всего записей: {books().length}</p>

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

            <Show when={books().length > 0} fallback={<p class={styles.empty}>Книг пока нет</p>}>
                <div class={styles.tableScroll}>
                    <table
                        class={styles.table}
                        aria-labelledby="books-title"
                        aria-label="Таблица книг"
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
                                <th scope="col">Название</th>
                                <th scope="col">Автор</th>
                                <th scope="col">Год</th>
                                <th scope="col">Инв. номер</th>
                                <th scope="col">Издательство</th>
                                <th scope="col">Создано (UTC)</th>
                                <th scope="col">Обновлено (UTC)</th>
                                <th scope="col">Удалено (UTC)</th>
                                <th scope="col">Действия</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For each={books()}>
                                {(book) => (
                                    <tr
                                        style={rowStyle(book.id)}
                                        onDblClick={(event) => handleRowDoubleClick(event, book.id)}
                                    >
                                        <td class={styles.checkboxColumn}>
                                            <input
                                                type="checkbox"
                                                checked={selectedIds().includes(book.id)}
                                                onChange={() => toggleRow(book.id)}
                                                aria-label={`Выбрать книгу ${book.title}`}
                                            />
                                        </td>
                                        <td>{book.title}</td>
                                        <td>{book.author}</td>
                                        <td>{book.year}</td>
                                        <td>{book.inventory_number}</td>
                                        <td>{publisherName(book.publisher_id)}</td>
                                        <td>{formatDate(book.created_at)}</td>
                                        <td>{formatDate(book.updated_at)}</td>
                                        <td>{formatDate(book.deleted_at)}</td>
                                        <td>
                                            <div class={styles.rowActions}>
                                                <button
                                                    id={`book-edit-button-${book.id}`}
                                                    class={styles.iconButton}
                                                    type="button"
                                                    onClick={() => openEdit(book)}
                                                    aria-label="Редактировать книгу"
                                                >
                                                    <FiEdit2 size={16} aria-hidden="true" />
                                                </button>
                                                <button
                                                    class={styles.iconButton}
                                                    type="button"
                                                    onClick={() => setDeletingBook(book)}
                                                    aria-label="Удалить книгу"
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

            <Show when={editingBook()}>
                <Modal title="Редактировать книгу" onClose={closeEdit}>
                    <form class={styles.form} onSubmit={saveEdit} noValidate>
                        <div class={styles.field}>
                            <label for="book-edit-publisher">Издательство</label>
                            <select
                                id="book-edit-publisher"
                                name="publisher_id"
                                value={editPublisherId()}
                                onInput={(event) => setEditPublisherId(event.currentTarget.value)}
                                aria-invalid={editErrors().publisher_id ? true : undefined}
                                aria-describedby={
                                    editErrors().publisher_id ? "book-edit-publisher-error" : undefined
                                }
                            >
                                <option value="">— выберите издательство —</option>
                                <For each={mockPublishers}>
                                    {(publisher) => (
                                        <option value={publisher.id}>{publisher.name}</option>
                                    )}
                                </For>
                            </select>
                            <Show when={editErrors().publisher_id}>
                                <p class={styles.error} id="book-edit-publisher-error" role="alert">
                                    {editErrors().publisher_id}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="book-edit-title">Название</label>
                            <input
                                id="book-edit-title"
                                name="title"
                                type="text"
                                required
                                autofocus
                                minLength={2}
                                maxLength={256}
                                value={editTitle()}
                                onInput={(event) => setEditTitle(event.currentTarget.value)}
                                aria-invalid={editErrors().title ? true : undefined}
                                aria-describedby={
                                    editErrors().title ? "book-edit-title-error" : undefined
                                }
                            />
                            <Show when={editErrors().title}>
                                <p class={styles.error} id="book-edit-title-error" role="alert">
                                    {editErrors().title}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="book-edit-author">Автор</label>
                            <input
                                id="book-edit-author"
                                name="author"
                                type="text"
                                required
                                minLength={2}
                                maxLength={256}
                                value={editAuthor()}
                                onInput={(event) => setEditAuthor(event.currentTarget.value)}
                                aria-invalid={editErrors().author ? true : undefined}
                                aria-describedby={
                                    editErrors().author ? "book-edit-author-error" : undefined
                                }
                            />
                            <Show when={editErrors().author}>
                                <p class={styles.error} id="book-edit-author-error" role="alert">
                                    {editErrors().author}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="book-edit-year">Год издания</label>
                            <input
                                id="book-edit-year"
                                name="year"
                                type="number"
                                required
                                min={1450}
                                max={2026}
                                step={1}
                                value={editYear()}
                                onInput={(event) => setEditYear(event.currentTarget.value)}
                                aria-invalid={editErrors().year ? true : undefined}
                                aria-describedby={
                                    editErrors().year ? "book-edit-year-error" : undefined
                                }
                            />
                            <Show when={editErrors().year}>
                                <p class={styles.error} id="book-edit-year-error" role="alert">
                                    {editErrors().year}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="book-edit-inventory">Инвентарный номер</label>
                            <input
                                id="book-edit-inventory"
                                name="inventory_number"
                                type="text"
                                required
                                minLength={3}
                                maxLength={64}
                                value={editInventory()}
                                onInput={(event) => setEditInventory(event.currentTarget.value)}
                                aria-invalid={editErrors().inventory_number ? true : undefined}
                                aria-describedby={
                                    editErrors().inventory_number
                                        ? "book-edit-inventory-error"
                                        : undefined
                                }
                            />
                            <Show when={editErrors().inventory_number}>
                                <p class={styles.error} id="book-edit-inventory-error" role="alert">
                                    {editErrors().inventory_number}
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

            <Show when={deletingBook()}>
                {(book) => (
                    <ConfirmDialog
                        title="Удаление книги"
                        message={`Удалить эту запись? «${book().title}»`}
                        confirmLabel="Удалить"
                        onConfirm={confirmDelete}
                        onCancel={() => setDeletingBook(null)}
                    />
                )}
            </Show>

            <Show when={booksToDelete().length > 0}>
                <ConfirmDialog
                    title="Массовое удаление"
                    message={`Удалить выбранные записи (${booksToDelete().length}): ${booksToDelete()
                        .map((book) => `«${book.title}»`)
                        .join(", ")}?`}
                    confirmLabel="Удалить"
                    onConfirm={confirmBulkDelete}
                    onCancel={cancelBulkDelete}
                />
            </Show>
        </div>
    );
}