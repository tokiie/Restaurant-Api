-- Add up migration script here
CREATE TABLE Device (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL
);

CREATE TABLE Tables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL
);

CREATE TABLE Menu (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    prepTime INTEGER NOT NULL
);


CREATE TABLE Items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tables_id UUID NOT NULL REFERENCES Tables(id),
    menu_id UUID NOT NULL REFERENCES Menu(id),
    quantity INTEGER NOT NULL,
    delivered_quantity INTEGER NOT NULL,
    delivered_at TIMESTAMP DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_by VARCHAR(255),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_by VARCHAR(255),
    deleted_at TIMESTAMP DEFAULT NULL,
    deleted_by VARCHAR(255)
);

