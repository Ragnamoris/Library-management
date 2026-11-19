export interface Publisher {
    id: string;
    name: string;
    address: string;
    created_at: string;
    updated_at: string | null;
    deleted_at: string | null;
}

export interface Book {
    id: string;
    publisher_id: string;
    title: string;
    author: string;
    year: number;
    inventory_number: string;
    created_at: string;
    updated_at: string | null;
    deleted_at: string | null;
}

export interface Reader {
    id: string;
    full_name: string;
    phone: string;
    card_number: string;
    created_at: string;
    updated_at: string | null;
    deleted_at: string | null;
}

export interface Loan {
    id: string;
    book_id: string;
    reader_id: string;
    issued_at: string;
    planned_return_at: string;
    returned_at: string | null;
    created_at: string;
    updated_at: string | null;
    deleted_at: string | null;
}