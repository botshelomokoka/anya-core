-- Create ML models table
CREATE TABLE ml_models (
    id SERIAL PRIMARY KEY,
    version VARCHAR(50) NOT NULL,
    features JSONB NOT NULL,
    weights JSONB NOT NULL,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    validation_score DOUBLE PRECISION NOT NULL
);

-- Create model features table
CREATE TABLE model_features (
    id SERIAL PRIMARY KEY,
    model_id INTEGER REFERENCES ml_models(id),
    name VARCHAR(255) NOT NULL,
    importance DOUBLE PRECISION NOT NULL,
    category VARCHAR(50) NOT NULL
);

-- Create audit logs table
CREATE TABLE audit_logs (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    action VARCHAR(50) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    details JSONB NOT NULL
);

-- Create file tracking table
CREATE TABLE file_tracking (
    id SERIAL PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    category VARCHAR(50) NOT NULL,
    importance_score DOUBLE PRECISION NOT NULL,
    last_analyzed TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB NOT NULL
);
