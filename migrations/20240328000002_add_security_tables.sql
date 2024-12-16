-- Add key management table
CREATE TABLE key_storage (
    id SERIAL PRIMARY KEY,
    key_id VARCHAR(64) NOT NULL UNIQUE,
    encrypted_key BYTEA NOT NULL,
    key_type VARCHAR(32) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_used TIMESTAMP WITH TIME ZONE,
    metadata JSONB
);

-- Add security events table
CREATE TABLE security_events (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(32) NOT NULL,
    severity VARCHAR(16) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    details JSONB NOT NULL,
    resolved BOOLEAN DEFAULT FALSE
);

-- Add model training history
CREATE TABLE model_training_history (
    id SERIAL PRIMARY KEY,
    model_id INTEGER REFERENCES ml_models(id),
    training_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    training_duration INTEGER NOT NULL, -- in seconds
    parameters JSONB NOT NULL,
    metrics JSONB NOT NULL,
    validation_results JSONB NOT NULL
);
