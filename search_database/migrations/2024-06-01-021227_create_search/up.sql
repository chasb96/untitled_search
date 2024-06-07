CREATE EXTENSION pg_trgm;
CREATE EXTENSION fuzzystrmatch;

CREATE TABLE search (
    result_type SMALLINT NOT NULL,
    value VARCHAR(32) NOT NULL,
    dmeta VARCHAR(64) NOT NULL
);

CREATE INDEX idx_search_dmeta_trgm ON search USING gin (dmeta gin_trgm_ops);