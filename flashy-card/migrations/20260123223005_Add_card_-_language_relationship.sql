
/**
* Add language id foreign key to cards.
*/
alter table cards add column if not exists language_id integer not null references languages (id);
