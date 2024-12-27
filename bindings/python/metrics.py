from dataclasses import dataclass
from datetime import datetime
from enum import Enum
from typing import Dict, Optional
import json

class SyncStatus(Enum):
    SYNCED = "Synced"
    SYNCING = "Syncing"
    BEHIND = "Behind"
    ERROR = "Error"

@dataclass
class SystemMetrics:
    cpu_usage: float
    memory_usage: float
    disk_usage: float
    network_traffic: float
    error_rate: float
    ops_total: int
    ops_success: int
    ops_failed: int
    ops_latency: float
    health_score: float
    last_check: datetime

@dataclass
class MLMetrics:
    model_accuracy: float
    training_time: float
    inference_time: float
    model_size: float
    dataset_size: int
    last_trained: datetime

@dataclass
class SecurityMetrics:
    vulnerability_count: int
    security_score: float
    last_audit: datetime
    critical_issues: int
    encryption_status: bool

@dataclass
class ProtocolMetrics:
    sync_status: SyncStatus
    block_height: int
    peer_count: int
    network_health: float
    last_block: datetime

@dataclass
class EnterpriseMetrics:
    transaction_count: int
    total_volume: float
    success_rate: float
    revenue: float
    active_users: int

@dataclass
class ValidationMetrics:
    validation_score: float
    error_count: int
    warning_count: int
    last_validation: datetime

@dataclass
class UnifiedMetrics:
    system: SystemMetrics
    ml: Optional[MLMetrics] = None
    security: Optional[SecurityMetrics] = None
    protocol: Optional[ProtocolMetrics] = None
    enterprise: Optional[EnterpriseMetrics] = None
    validation: Optional[ValidationMetrics] = None
    custom: Dict[str, float] = None
    timestamp: datetime = None

    def __post_init__(self):
        if self.custom is None:
            self.custom = {}
        if self.timestamp is None:
            self.timestamp = datetime.utcnow()

@dataclass
class ComponentHealth:
    operational: bool
    health_score: float
    last_incident: Optional[datetime]
    error_count: int
    warning_count: int

class MetricsError(Exception):
    """Base class for metrics-related errors"""
    pass

class MetricsManager:
    def __init__(self):
        self._metrics = UnifiedMetrics(
            system=SystemMetrics(
                cpu_usage=0.0,
                memory_usage=0.0,
                disk_usage=0.0,
                network_traffic=0.0,
                error_rate=0.0,
                ops_total=0,
                ops_success=0,
                ops_failed=0,
                ops_latency=0.0,
                health_score=100.0,
                last_check=datetime.utcnow()
            )
        )
        self._counters = {}
        self._gauges = {}
        self._histograms = {}

    def register_counter(self, name: str, help: str) -> None:
        """Register a new counter metric"""
        if name in self._counters:
            raise MetricsError(f"Counter {name} already exists")
        self._counters[name] = 0

    def register_gauge(self, name: str, help: str) -> None:
        """Register a new gauge metric"""
        if name in self._gauges:
            raise MetricsError(f"Gauge {name} already exists")
        self._gauges[name] = 0.0

    def register_histogram(self, name: str, help: str) -> None:
        """Register a new histogram metric"""
        if name in self._histograms:
            raise MetricsError(f"Histogram {name} already exists")
        self._histograms[name] = []

    def update_metrics(self, metrics: UnifiedMetrics) -> None:
        """Update the unified metrics"""
        self._metrics = metrics

    def get_metrics(self) -> UnifiedMetrics:
        """Get the current unified metrics"""
        return self._metrics

    def increment_counter(self, name: str) -> None:
        """Increment a counter metric"""
        if name not in self._counters:
            raise MetricsError(f"Counter {name} not found")
        self._counters[name] += 1

    def set_gauge(self, name: str, value: float) -> None:
        """Set a gauge metric value"""
        if name not in self._gauges:
            raise MetricsError(f"Gauge {name} not found")
        self._gauges[name] = value

    def observe_histogram(self, name: str, value: float) -> None:
        """Add an observation to a histogram metric"""
        if name not in self._histograms:
            raise MetricsError(f"Histogram {name} not found")
        self._histograms[name].append(value)

    def to_json(self) -> str:
        """Convert metrics to JSON string"""
        return json.dumps(self._metrics, default=lambda o: o.__dict__)

    @classmethod
    def from_json(cls, json_str: str) -> 'MetricsManager':
        """Create MetricsManager from JSON string"""
        data = json.loads(json_str)
        manager = cls()
        metrics = UnifiedMetrics(**data)
        manager.update_metrics(metrics)
        return manager
