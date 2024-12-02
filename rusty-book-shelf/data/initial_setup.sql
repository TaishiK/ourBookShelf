INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig', 
    'eleazar.fig@example.com',
    '$2b$12$vlJfFk5EBJFACvRx/0iOquBu7To9Qon6VxpFI26xLBgVxk5XDpyOe',
    role_id
FROM
    roles
WHERE
    name Like 'Admin';

INSERT INTO
    books (title, author, isbn, description, user_id, created_at)
SELECT
    'The Rust Programming Language',
    'Steve Klabnik and Carol Nichols',
    '9781593278281',
    'The Rust Programming Language is the official book on Rust: an open source systems programming language that helps you write faster, more reliable software. Rust offers control over low-level details (such as memory usage) in combination with high-level ergonomics, eliminating the hassle traditionally associated with low-level languages.',
    user_id,
    NOW()
FROM
    users
WHERE
    name Like 'Eleazar Fig';

