-- Add up migration script here
INSERT INTO tables (id, name) VALUES
    (gen_random_uuid(), 'Table 1'),
    (gen_random_uuid(), 'Table 2');

-- Insert sample data for Devices
INSERT INTO device (id, name) VALUES
    (gen_random_uuid(), 'Device 1'),
    (gen_random_uuid(), 'Device 2');

-- Insert sample data for Menu
INSERT INTO menu (id, name, price, prep_time) VALUES
    (gen_random_uuid(), 'Burger', 5.99, 5),
    (gen_random_uuid(), 'Pizza', 8.99, 8);