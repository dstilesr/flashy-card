-- Add migration script here

-- Add slug column to card_decks for URL-friendly routing
ALTER TABLE card_decks ADD COLUMN slug VARCHAR(128);

-- Populate existing rows (convert name to slug format if any decks exist)
UPDATE card_decks SET slug = LOWER(REPLACE(REPLACE(name, ' ', '-'), '_', '-')) WHERE slug IS NULL;

-- Make slug NOT NULL and add unique constraint per language
ALTER TABLE card_decks ALTER COLUMN slug SET NOT NULL;
CREATE UNIQUE INDEX card_decks_slug_language ON card_decks (slug, language_id);
