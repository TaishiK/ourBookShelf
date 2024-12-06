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
    users (name, email, password_hash, role_id)
SELECT
    '九里大志',
    'Taishi.Kunori@sony.com',
    '$2b$12$vlJfFk5EBJFACvRx/0iOquBu7To9Qon6VxpFI26xLBgVxk5XDpyOe',
    role_id
FROM
    roles
WHERE
    name Like 'User';

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    '九里大志',
    'Taishi.Kunori@sony.com',
    '$2b$12$vlJfFk5EBJFACvRx/0iOquBu7To9Qon6VxpFI26xLBgVxk5XDpyOe',
    role_id
FROM
    roles
WHERE
    name Like 'User';

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
INSERT INTO
    books (title, author, isbn, description, user_id, created_at)
SELECT
    'RustによるWebアプリケーション開発　設計からリリース・運用まで',
    '豊田勇貴・松本健太郎・吉川哲史',
    '9784065369579'
    '★Rustによるアプリケーション開発のベストプラクティス！
Rustを現場で使うときがきた! Rust経験豊富な筆者が、貴重な知識とテクニックを惜しみなく伝授。
「蔵書管理アプリケーション」の実装を通じて、Rustによる設計、開発、保守、運用までをハンズオンで学ぶ！
コードも丁寧に解説。',
    user_id,
    NOW()
FROMq
    users
WHERE
    name Like 'Eleazar Fig';

INSERT INTO
    books (title, author, isbn, description, user_id, created_at)
SELECT
    'RustによるWebアプリケーション開発 設計からリリース・運用まで',
    '豊田優貴・松本健太郎・吉川哲史',
    '9784065369579',
    'Rustによるアプリケーション開発のベストプラクティス! Rustを現場で使うときがきた!',
    user_id,
    NOW()
FROM
    users
WHERE
    name Like 'Eleazar Fig';
