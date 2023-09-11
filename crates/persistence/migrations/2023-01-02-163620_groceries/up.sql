-- Your SQL goes here
CREATE TABLE checklist (
	"item_id"	INTEGER NOT NULL UNIQUE,
	PRIMARY KEY("item_id"),
	FOREIGN KEY("item_id") REFERENCES "items"("id")
);

CREATE TABLE items (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE items_recipes (
	"item_id"	INTEGER NOT NULL,
	"recipe_id"	INTEGER NOT NULL,
	PRIMARY KEY("item_id"),
	FOREIGN KEY("item_id") REFERENCES "items"("id"),
	FOREIGN KEY("recipe_id") REFERENCES "recipes"("id")
);

CREATE TABLE items_sections (
	"item_id"	INTEGER NOT NULL UNIQUE,
	"section_id"	INTEGER NOT NULL,
	PRIMARY KEY("item_id"),
	FOREIGN KEY("item_id") REFERENCES "items"("id"),
	FOREIGN KEY("section_id") REFERENCES "sections"("id")
);

CREATE TABLE list (
	"item_id"	INTEGER NOT NULL UNIQUE,
	PRIMARY KEY("item_id"),
	FOREIGN KEY("item_id") REFERENCES "items"("id")
);

CREATE TABLE recipes (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE sections (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
