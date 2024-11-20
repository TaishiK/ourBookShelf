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
