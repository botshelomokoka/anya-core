# API Reference

## Core API

### Authentication
```rust
POST /api/v1/auth/login
POST /api/v1/auth/refresh
POST /api/v1/auth/logout
```

### User Management
```rust
GET /api/v1/users
POST /api/v1/users
PUT /api/v1/users/{id}
DELETE /api/v1/users/{id}
```

### Blockchain Operations
```rust
POST /api/v1/blockchain/transaction
GET /api/v1/blockchain/status
GET /api/v1/blockchain/block/{hash}
```

## Dashboard API

### Metrics
```rust
GET /api/v1/metrics/summary
GET /api/v1/metrics/detailed
POST /api/v1/metrics/custom
```

### Analytics
```rust
GET /api/v1/analytics/trends
GET /api/v1/analytics/predictions
POST /api/v1/analytics/report
```

## Enterprise API

### Integration
```rust
POST /api/v1/enterprise/connect
GET /api/v1/enterprise/status
PUT /api/v1/enterprise/config
```

### Compliance
```rust
GET /api/v1/compliance/audit
POST /api/v1/compliance/report
GET /api/v1/compliance/status
```

## Mobile API

### Sync
```rust
POST /api/v1/mobile/sync
GET /api/v1/mobile/status
PUT /api/v1/mobile/preferences
```

### Notifications
```rust
POST /api/v1/notifications/send
GET /api/v1/notifications/status
DELETE /api/v1/notifications/{id}
```

## Error Codes

| Code | Description |
|------|-------------|
| 200  | Success |
| 400  | Bad Request |
| 401  | Unauthorized |
| 403  | Forbidden |
| 404  | Not Found |
| 500  | Server Error |

## Rate Limits
- Standard: 100 requests/minute
- Enterprise: 1000 requests/minute
- Custom: Configurable
