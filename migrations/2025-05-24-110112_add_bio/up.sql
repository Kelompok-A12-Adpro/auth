-- Your SQL goes here
ALTER TABLE users
  ADD COLUMN bio VARCHAR(255) NOT NULL
    DEFAULT 'Aku bersedia membagikan cintaku kepada dunia 💗';
