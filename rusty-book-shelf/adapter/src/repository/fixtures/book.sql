INSERT INTO books (
    book_id,
    title,
    author,
    isbn,
    description,
    user_id,
    created_at,
    updated_at
)
VALUES (
    '987654321',
    '実践Rustプログラミング入門',
    '初田直也他',
    '978-4798061702',
    'C/C++の代わりとなるべき最新言語その独特な仕様をわかりやすく解説',
    '123456789',
    now(),
    now()
)
ON CONFLICT DO NOTHING;