-- Add migration script here
/**
* Table to hold the languages to study.
*/
CREATE TABLE languages (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL ,
    slug VARCHAR(64) NOT NULL ,
    description VARCHAR(256),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
);

create unique index languages_slug on languages (slug);

/**
* Table to hold card decks. A card deck is a collection of cards to study for a given language.
* The cards in a deck should have a common theme of some kind.
*/
CREATE TABLE card_decks (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL ,
    description VARCHAR(256),
    language_id INTEGER NOT NULL REFERENCES languages(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
);

/*
* Holds the types of cards that can be created. These can be either parts of speech or
* phrases or idioms in the language.
*/
CREATE TABLE card_types (
    id integer not null primary key,
    type_name varchar(64) not null
);

-- Pre-populate this table
insert into card_types (id, type_name) values
    (1, 'Noun'),
    (2, 'Verb'),
    (3, 'Adjective'),
    (4, 'Adverb'),
    (5, 'Conjunction'),
    (6, 'Preposition'),
    (7, 'Root'),
    (8, 'Idiom'),
    (9, 'Pronoun'),
    (10, 'Phrase'),
    (11, 'Numeral')
;


/**
* Table to hold cards for study in a language. A card will have:
* - A target in the target language
* - A hint to help with guessing
* - A translation in a known language
* - Some examples in the target language
* - Additional information, such as grammatical information
*/
CREATE TABLE cards (
    id serial primary key,
    type_id integer not null references card_types(id),
    target varchar(1024) not null,
    hint varchar(2048),
    translation varchar(2048) not null,
    examples text,
    additional_info text,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
);


/**
* Map cards to their decks (many-to-many relationship)
*/
create table card_to_deck (
    card_id integer not null references cards(id),
    deck_id integer not null references card_decks(id),
    primary key (card_id, deck_id)
);
