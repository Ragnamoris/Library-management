import { For, Show, createSignal } from "solid-js";
import type { JSX } from "solid-js";
import { FiCheck, FiEdit2, FiPlus, FiTrash2 } from "solid-icons/fi";
import { mockBooks, mockLoans, mockReaders } from "../data/mocks";
import type { Loan } from "../data/types";
import { formatDate } from "../lib/formatDate";
import { ConfirmDialog } from "../ui/modal/ConfirmDialog";
import { Modal } from "../ui/modal/Modal";
import styles from "../ui/pages.module.css";

interface LoanFormErrors {
    book_id?: string;
    reader_id?: string;
    issued_at?: string;
    returned_at?: string;
}

const LOAN_TERM_DAYS = 20;
const READER_LIMIT = 5;

function toIsoDate(dateValue: string): string {
    return `${dateValue}T00:00:00.000Z`;
}

function addDaysIso(dateValue: string, days: number): string {
    const date = new Date(toIsoDate(dateValue));
    date.setUTCDate(date.getUTCDate() + days);
    return date.toISOString();
}

function todayValue(): string {
    return new Date().toISOString().slice(0, 10);
}

export function LoansPage(): JSX.Element {
    const [loans, setLoans] = createSignal<Loan[]>(mockLoans);
    const [bookId, setBookId] = createSignal("");
    const [readerId, setReaderId] = createSignal("");
    const [issueDate, setIssueDate] = createSignal(todayValue());
    const [errors, setErrors] = createSignal<LoanFormErrors>({});
    const [feedback, setFeedback] = createSignal("");
    const [editingLoan, setEditingLoan] = createSignal<Loan | null>(null);
    const [deletingLoan, setDeletingLoan] = createSignal<Loan | null>(null);
    const [editBookId, setEditBookId] = createSignal("");
    const [editReaderId, setEditReaderId] = createSignal("");
    const [editIssueDate, setEditIssueDate] = createSignal("");
    const [editReturnDate, setEditReturnDate] = createSignal("");
    const [editErrors, setEditErrors] = createSignal<LoanFormErrors>({});
    const [selectedIds, setSelectedIds] = createSignal<string[]>([]);
    const [bulkDeleteOpen, setBulkDeleteOpen] = createSignal(false);

    function bookLabel(id: string): string {
        const book = mockBooks.find((item) => item.id === id);
        return book ? `${book.title} (${book.inventory_number})` : "Неизвестная книга";
    }

    function readerLabel(id: string): string {
        const reader = mockReaders.find((item) => item.id === id);
        return reader ? reader.full_name : "Неизвестный читатель";
    }

    function isBookOnHand(selectedBookId: string, excludeLoanId?: string): boolean {
        return loans().some(
            (loan) =>
                loan.book_id === selectedBookId &&
                loan.returned_at === null &&
                loan.id !== excludeLoanId,
        );
    }

    function activeLoansForReader(selectedReaderId: string, excludeLoanId?: string): number {
        return loans().filter(
            (loan) =>
                loan.reader_id === selectedReaderId &&
                loan.returned_at === null &&
                loan.id !== excludeLoanId,
        ).length;
    }

    function loanStatus(loan: Loan): "active" | "overdue" | "returned" {
        if (loan.returned_at !== null) {
            return "returned";
        }
        return loan.planned_return_at < new Date().toISOString() ? "overdue" : "active";
    }

    function statusLabel(loan: Loan): string {
        const status = loanStatus(loan);
        if (status === "overdue") {
            return "Просрочена";
        }
        if (status === "returned") {
            return "Возвращена";
        }
        return "На руках";
    }

    function statusClass(loan: Loan): string {
        const status = loanStatus(loan);
        if (status === "overdue") {
            return styles.statusOverdue;
        }
        if (status === "returned") {
            return styles.statusReturned;
        }
        return styles.statusActive;
    }

    function allSelected(): boolean {
        const ids = selectedIds();
        return loans().length > 0 && loans().every((loan) => ids.includes(loan.id));
    }

    function toggleAll(): void {
        if (allSelected()) {
            setSelectedIds([]);
        } else {
            setSelectedIds(loans().map((loan) => loan.id));
        }
    }

    function toggleRow(id: string): void {
        setSelectedIds((current) =>
            current.includes(id)
                ? current.filter((item) => item !== id)
                : [...current, id],
        );
    }

    function addLoan(event: SubmitEvent): void {
        event.preventDefault();
        const nextBookId = bookId();
        const nextReaderId = readerId();
        const nextIssueDate = issueDate();
        const nextErrors: LoanFormErrors = {};
        if (nextBookId === "") {
            nextErrors.book_id = "Выберите книгу из списка";
        } else if (isBookOnHand(nextBookId)) {
            nextErrors.book_id = "Эта книга уже на руках и не может быть выдана повторно";
        }
        if (nextReaderId === "") {
            nextErrors.reader_id = "Выберите читателя из списка";
        } else if (activeLoansForReader(nextReaderId) >= READER_LIMIT) {
            nextErrors.reader_id = `У читателя уже ${READER_LIMIT} книг на руках — предел достигнут`;
        }
        if (nextIssueDate === "") {
            nextErrors.issued_at = "Укажите дату выдачи";
        }
        setErrors(nextErrors);
        if (nextErrors.book_id || nextErrors.reader_id || nextErrors.issued_at) {
            return;
        }
        const newLoan: Loan = {
            id: crypto.randomUUID(),
            book_id: nextBookId,
            reader_id: nextReaderId,
            issued_at: toIsoDate(nextIssueDate),
            planned_return_at: addDaysIso(nextIssueDate, LOAN_TERM_DAYS),
            returned_at: null,
            created_at: new Date().toISOString(),
            updated_at: null,
            deleted_at: null,
        };
        setLoans((currentLoans) => [...currentLoans, newLoan]);
        setBookId("");
        setReaderId("");
        setIssueDate(todayValue());
        setFeedback("Выдача оформлена");
    }

    function openEdit(loan: Loan): void {
        setEditingLoan(loan);
        setEditBookId(loan.book_id);
        setEditReaderId(loan.reader_id);
        setEditIssueDate(loan.issued_at.slice(0, 10));
        setEditReturnDate(loan.returned_at === null ? "" : loan.returned_at.slice(0, 10));
        setEditErrors({});
        setFeedback("");
    }

    function handleRowDoubleClick(event: MouseEvent, id: string): void {
        const target = event.target;
        if (target instanceof Element && target.closest("input, button, select, a")) {
            return;
        }
        toggleRow(id);
    }

    function closeEdit(): void {
        setEditingLoan(null);
    }

    function saveEdit(event: SubmitEvent): void {
        event.preventDefault();
        const target = editingLoan();
        if (!target) {
            return;
        }
        const nextBookId = editBookId();
        const nextReaderId = editReaderId();
        const nextIssueDate = editIssueDate();
        const nextReturnDate = editReturnDate();
        const nextErrors: LoanFormErrors = {};
        if (nextBookId === "") {
            nextErrors.book_id = "Выберите книгу из списка";
        } else if (isBookOnHand(nextBookId, target.id)) {
            nextErrors.book_id = "Эта книга уже на руках и не может быть выдана повторно";
        }
        if (nextReaderId === "") {
            nextErrors.reader_id = "Выберите читателя из списка";
        } else if (activeLoansForReader(nextReaderId, target.id) >= READER_LIMIT) {
            nextErrors.reader_id = `У читателя уже ${READER_LIMIT} книг на руках — предел достигнут`;
        }
        if (nextIssueDate === "") {
            nextErrors.issued_at = "Укажите дату выдачи";
        }
        if (nextReturnDate !== "" && nextIssueDate !== "" && nextReturnDate < nextIssueDate) {
            nextErrors.returned_at = "Дата возврата не может быть раньше даты выдачи";
        }
        setEditErrors(nextErrors);
        if (
            nextErrors.book_id ||
            nextErrors.reader_id ||
            nextErrors.issued_at ||
            nextErrors.returned_at
        ) {
            return;
        }
        setLoans((currentLoans) =>
            currentLoans.map((loan) =>
                loan.id === target.id
                    ? {
                          ...loan,
                          book_id: nextBookId,
                          reader_id: nextReaderId,
                          issued_at: toIsoDate(nextIssueDate),
                          planned_return_at: addDaysIso(nextIssueDate, LOAN_TERM_DAYS),
                          returned_at: nextReturnDate === "" ? null : toIsoDate(nextReturnDate),
                          updated_at: new Date().toISOString(),
                      }
                    : loan,
            ),
        );
        setEditingLoan(null);
        setFeedback("Изменения сохранены");
    }

    function confirmDelete(): void {
        const target = deletingLoan();
        if (!target) {
            return;
        }
        setLoans((currentLoans) => currentLoans.filter((loan) => loan.id !== target.id));
        setSelectedIds((currentIds) => currentIds.filter((id) => id !== target.id));
        setDeletingLoan(null);
        setFeedback("Запись удалена");
    }

    function confirmBulkDelete(): void {
        const ids = selectedIds();
        if (ids.length === 0) {
            setBulkDeleteOpen(false);
            return;
        }
        setLoans((currentLoans) => currentLoans.filter((loan) => !ids.includes(loan.id)));
        setSelectedIds([]);
        setBulkDeleteOpen(false);
        setFeedback(`Удалено записей: ${ids.length}`);
    }

    return (
        <div class={styles.page}>
            <h1 class={styles.title} id="loans-title">Выдачи</h1>

            <form class={styles.form} aria-labelledby="loans-title" onSubmit={addLoan} noValidate>
                <div class={styles.field}>
                    <label for="loan-book">Книга</label>
                    <select
                        id="loan-book"
                        name="book_id"
                        value={bookId()}
                        onInput={(event) => {
                            setBookId(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().book_id ? true : undefined}
                        aria-describedby={errors().book_id ? "loan-book-error" : undefined}
                    >
                        <option value="">— выберите книгу —</option>
                        <For each={mockBooks}>
                            {(book) => (
                                <option value={book.id}>
                                    {book.title} ({book.inventory_number})
                                    {isBookOnHand(book.id) ? " — на руках" : ""}
                                </option>
                            )}
                        </For>
                    </select>
                    <Show when={errors().book_id}>
                        <p class={styles.error} id="loan-book-error" role="alert">
                            {errors().book_id}
                        </p>
                    </Show>
                </div>

                <div class={styles.field}>
                    <label for="loan-reader">Читатель</label>
                    <select
                        id="loan-reader"
                        name="reader_id"
                        value={readerId()}
                        onInput={(event) => {
                            setReaderId(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().reader_id ? true : undefined}
                        aria-describedby={errors().reader_id ? "loan-reader-error" : undefined}
                    >
                        <option value="">— выберите читателя —</option>
                        <For each={mockReaders}>
                            {(reader) => (
                                <option value={reader.id}>
                                    {reader.full_name} — на руках:{" "}
                                    {activeLoansForReader(reader.id)}/{READER_LIMIT}
                                </option>
                            )}
                        </For>
                    </select>
                    <Show when={errors().reader_id}>
                        <p class={styles.error} id="loan-reader-error" role="alert">
                            {errors().reader_id}
                        </p>
                    </Show>
                </div>

                <div class={styles.field}>
                    <label for="loan-issue-date">Дата выдачи</label>
                    <input
                        id="loan-issue-date"
                        name="issued_at"
                        type="date"
                        required
                        value={issueDate()}
                        onInput={(event) => {
                            setIssueDate(event.currentTarget.value);
                            setFeedback("");
                        }}
                        aria-invalid={errors().issued_at ? true : undefined}
                        aria-describedby={errors().issued_at ? "loan-issue-date-error" : undefined}
                    />
                    <Show when={errors().issued_at}>
                        <p class={styles.error} id="loan-issue-date-error" role="alert">
                            {errors().issued_at}
                        </p>
                    </Show>
                    <Show when={issueDate() !== ""}>
                        <p class={styles.hint}>
                            Плановая дата возврата: {formatDate(addDaysIso(issueDate(), LOAN_TERM_DAYS))}{" "}
                            (срок пользования — {LOAN_TERM_DAYS} дней)
                        </p>
                    </Show>
                </div>

                <div class={styles.formActions}>
                    <button class={styles.submitButton} type="submit">
                        <FiPlus size={16} aria-hidden="true" />
                        Оформить выдачу
                    </button>
                </div>

                <Show when={feedback()}>
                    <p class={styles.feedback} role="status">
                        {feedback()}
                    </p>
                </Show>
            </form>

            <p class={styles.counter}>Всего записей: {loans().length}</p>

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

            <Show when={loans().length > 0} fallback={<p class={styles.empty}>Выдач пока нет</p>}>
                <div class={styles.tableScroll}>
                    <table
                        class={styles.table}
                        aria-labelledby="loans-title"
                        aria-label="Таблица выдач"
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
                                <th scope="col">Книга</th>
                                <th scope="col">Читатель</th>
                                <th scope="col">Выдача (UTC)</th>
                                <th scope="col">Плановый возврат (UTC)</th>
                                <th scope="col">Возврат (UTC)</th>
                                <th scope="col">Статус</th>
                                <th scope="col">Создано (UTC)</th>
                                <th scope="col">Обновлено (UTC)</th>
                                <th scope="col">Удалено (UTC)</th>
                                <th scope="col">Действия</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For each={loans()}>
                                {(loan) => (
                                    <tr onDblClick={(event) => handleRowDoubleClick(event, loan.id)}>
                                        <td class={styles.checkboxColumn}>
                                            <input
                                                type="checkbox"
                                                checked={selectedIds().includes(loan.id)}
                                                onChange={() => toggleRow(loan.id)}
                                                aria-label={`Выбрать выдачу книги ${bookLabel(loan.book_id)}`}
                                            />
                                        </td>
                                        <td>{bookLabel(loan.book_id)}</td>
                                        <td>{readerLabel(loan.reader_id)}</td>
                                        <td>{formatDate(loan.issued_at)}</td>
                                        <td>{formatDate(loan.planned_return_at)}</td>
                                        <td>{formatDate(loan.returned_at)}</td>
                                        <td>
                                            <span class={statusClass(loan)}>{statusLabel(loan)}</span>
                                        </td>
                                        <td>{formatDate(loan.created_at)}</td>
                                        <td>{formatDate(loan.updated_at)}</td>
                                        <td>{formatDate(loan.deleted_at)}</td>
                                        <td>
                                            <div class={styles.rowActions}>
                                                <button
                                                    class={styles.iconButton}
                                                    type="button"
                                                    onClick={() => openEdit(loan)}
                                                    aria-label="Редактировать выдачу"
                                                >
                                                    <FiEdit2 size={16} aria-hidden="true" />
                                                </button>
                                                <button
                                                    class={styles.iconButton}
                                                    type="button"
                                                    onClick={() => setDeletingLoan(loan)}
                                                    aria-label="Удалить выдачу"
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

            <Show when={editingLoan()}>
                <Modal title="Редактировать выдачу" onClose={closeEdit}>
                    <form class={styles.form} onSubmit={saveEdit} noValidate>
                        <div class={styles.field}>
                            <label for="loan-edit-book">Книга</label>
                            <select
                                id="loan-edit-book"
                                name="book_id"
                                value={editBookId()}
                                onInput={(event) => setEditBookId(event.currentTarget.value)}
                                aria-invalid={editErrors().book_id ? true : undefined}
                                aria-describedby={
                                    editErrors().book_id ? "loan-edit-book-error" : undefined
                                }
                            >
                                <option value="">— выберите книгу —</option>
                                <For each={mockBooks}>
                                    {(book) => (
                                        <option value={book.id}>
                                            {book.title} ({book.inventory_number})
                                            {isBookOnHand(book.id, editingLoan()?.id)
                                                ? " — на руках"
                                                : ""}
                                        </option>
                                    )}
                                </For>
                            </select>
                            <Show when={editErrors().book_id}>
                                <p class={styles.error} id="loan-edit-book-error" role="alert">
                                    {editErrors().book_id}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="loan-edit-reader">Читатель</label>
                            <select
                                id="loan-edit-reader"
                                name="reader_id"
                                value={editReaderId()}
                                onInput={(event) => setEditReaderId(event.currentTarget.value)}
                                aria-invalid={editErrors().reader_id ? true : undefined}
                                aria-describedby={
                                    editErrors().reader_id ? "loan-edit-reader-error" : undefined
                                }
                            >
                                <option value="">— выберите читателя —</option>
                                <For each={mockReaders}>
                                    {(reader) => (
                                        <option value={reader.id}>
                                            {reader.full_name} — на руках:{" "}
                                            {activeLoansForReader(reader.id, editingLoan()?.id)}/
                                            {READER_LIMIT}
                                        </option>
                                    )}
                                </For>
                            </select>
                            <Show when={editErrors().reader_id}>
                                <p class={styles.error} id="loan-edit-reader-error" role="alert">
                                    {editErrors().reader_id}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="loan-edit-issue-date">Дата выдачи</label>
                            <input
                                id="loan-edit-issue-date"
                                name="issued_at"
                                type="date"
                                required
                                value={editIssueDate()}
                                onInput={(event) => setEditIssueDate(event.currentTarget.value)}
                                aria-invalid={editErrors().issued_at ? true : undefined}
                                aria-describedby={
                                    editErrors().issued_at
                                        ? "loan-edit-issue-date-error"
                                        : undefined
                                }
                            />
                            <Show when={editErrors().issued_at}>
                                <p class={styles.error} id="loan-edit-issue-date-error" role="alert">
                                    {editErrors().issued_at}
                                </p>
                            </Show>
                            <Show when={editIssueDate() !== ""}>
                                <p class={styles.hint}>
                                    Плановая дата возврата:{" "}
                                    {formatDate(addDaysIso(editIssueDate(), LOAN_TERM_DAYS))}
                                </p>
                            </Show>
                        </div>

                        <div class={styles.field}>
                            <label for="loan-edit-return-date">Дата возврата (пусто = на руках)</label>
                            <input
                                id="loan-edit-return-date"
                                name="returned_at"
                                type="date"
                                value={editReturnDate()}
                                onInput={(event) => setEditReturnDate(event.currentTarget.value)}
                                aria-invalid={editErrors().returned_at ? true : undefined}
                                aria-describedby={
                                    editErrors().returned_at
                                        ? "loan-edit-return-date-error"
                                        : undefined
                                }
                            />
                            <Show when={editErrors().returned_at}>
                                <p
                                    class={styles.error}
                                    id="loan-edit-return-date-error"
                                    role="alert"
                                >
                                    {editErrors().returned_at}
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

            <Show when={deletingLoan()}>
                {(loan) => (
                    <ConfirmDialog
                        title="Удаление выдачи"
                        message={`Удалить эту запись? Книга «${bookLabel(loan().book_id)}», читатель «${readerLabel(loan().reader_id)}»`}
                        confirmLabel="Удалить"
                        onConfirm={confirmDelete}
                        onCancel={() => setDeletingLoan(null)}
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