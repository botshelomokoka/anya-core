-- Add identity credentials table
CREATE TABLE identity_credentials (
    id SERIAL PRIMARY KEY,
    did VARCHAR(255) NOT NULL,
    credential_type VARCHAR(64) NOT NULL,
    credential_data JSONB NOT NULL,
    issued_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE,
    revoked BOOLEAN DEFAULT FALSE,
    metadata JSONB
);

-- Add verification records table
CREATE TABLE verification_records (
    id SERIAL PRIMARY KEY,
    credential_id INTEGER REFERENCES identity_credentials(id),
    verifier_did VARCHAR(255) NOT NULL,
    verification_method VARCHAR(64) NOT NULL,
    verified_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    verification_result BOOLEAN NOT NULL,
    proof JSONB NOT NULL
);

-- Add DID resolution cache
CREATE TABLE did_resolution_cache (
    id SERIAL PRIMARY KEY,
    did VARCHAR(255) NOT NULL UNIQUE,
    resolution_result JSONB NOT NULL,
    last_resolved TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    valid_until TIMESTAMP WITH TIME ZONE,
    metadata JSONB
);

-- Add indices
CREATE INDEX idx_identity_credentials_did ON identity_credentials(did);
CREATE INDEX idx_verification_records_credential ON verification_records(credential_id);
CREATE INDEX idx_did_resolution_cache_did ON did_resolution_cache(did);
