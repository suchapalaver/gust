CREATE TABLE items (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE recipes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE sections (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE checklist (
    item_id INTEGER NOT NULL,
    PRIMARY KEY (item_id),
    FOREIGN KEY (item_id) REFERENCES items (id)
);

CREATE TABLE list (
    item_id INTEGER NOT NULL,
    PRIMARY KEY (item_id),
    FOREIGN KEY (item_id) REFERENCES items (id)
);

CREATE TABLE items_recipes (
    item_id INTEGER NOT NULL,
    recipe_id INTEGER NOT NULL,
    PRIMARY KEY (item_id, recipe_id),
    FOREIGN KEY (item_id) REFERENCES items (id),
    FOREIGN KEY (recipe_id) REFERENCES recipes (id)
);

CREATE TABLE items_sections (
    item_id INTEGER NOT NULL,
    section_id INTEGER NOT NULL,
    PRIMARY KEY (item_id, section_id),
    FOREIGN KEY (item_id) REFERENCES items (id),
    FOREIGN KEY (section_id) REFERENCES sections (id)
);
