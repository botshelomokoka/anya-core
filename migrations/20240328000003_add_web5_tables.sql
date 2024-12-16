-- Add Web5 records table
CREATE TABLE web5_records (
    id SERIAL PRIMARY KEY,
    record_id VARCHAR(64) NOT NULL UNIQUE,
    protocol_id VARCHAR(255) NOT NULL,
    schema_id VARCHAR(255) NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    sync_status VARCHAR(32) NOT NULL DEFAULT 'pending'
);

-- Add DID documents table
CREATE TABLE did_documents (
    id SERIAL PRIMARY KEY,
    did VARCHAR(255) NOT NULL UNIQUE,
    document JSONB NOT NULL,
    last_resolved TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Add protocol definitions table
CREATE TABLE protocol_definitions (
    id SERIAL PRIMARY KEY,
    protocol_id VARCHAR(255) NOT NULL UNIQUE,
    definition JSONB NOT NULL,
    version VARCHAR(32) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
