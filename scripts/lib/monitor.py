"""
Unified monitoring system for Anya.
Handles system monitoring, resource tracking, and performance metrics.
"""

import os
import sys
import time
import psutil
import logging
from pathlib import Path
from typing import Dict, List, Optional, Any
from dataclasses import dataclass
from datetime import datetime
import json
from .utils import SystemInfo, Logger
from .project_config import ProjectConfig

@dataclass
class SystemMetrics:
    """System performance metrics."""
    cpu_percent: float
    memory_percent: float
    disk_usage_percent: float
    network_io: Dict[str, int]
    timestamp: datetime

@dataclass
class ProcessMetrics:
    """Process-specific metrics."""
    pid: int
    cpu_percent: float
    memory_percent: float
    threads: int
    open_files: int
    connections: int
    timestamp: datetime

class Monitor:
    """Base monitoring class."""
    
    def __init__(self, project_root: Optional[Path] = None):
        self.project_root = project_root or Path(__file__).parent.parent.parent
        self.config = ProjectConfig(self.project_root)
        self.system_info = SystemInfo()
        self.logger = Logger(self.project_root / 'logs')
        self.metrics_dir = self.project_root / 'logs' / 'metrics'
        self.metrics_dir.mkdir(parents=True, exist_ok=True)
    
    def collect_system_metrics(self) -> SystemMetrics:
        """Collect system-wide metrics."""
        return SystemMetrics(
            cpu_percent=psutil.cpu_percent(interval=1),
            memory_percent=psutil.virtual_memory().percent,
            disk_usage_percent=psutil.disk_usage('/').percent,
            network_io={
                'bytes_sent': psutil.net_io_counters().bytes_sent,
                'bytes_recv': psutil.net_io_counters().bytes_recv
            },
            timestamp=datetime.now()
        )
    
    def collect_process_metrics(self, pid: Optional[int] = None) -> ProcessMetrics:
        """Collect process-specific metrics."""
        if pid is None:
            pid = os.getpid()
        
        process = psutil.Process(pid)
        return ProcessMetrics(
            pid=pid,
            cpu_percent=process.cpu_percent(interval=1),
            memory_percent=process.memory_percent(),
            threads=process.num_threads(),
            open_files=len(process.open_files()),
            connections=len(process.connections()),
            timestamp=datetime.now()
        )
    
    def save_metrics(self, metrics: Any):
        """Save metrics to file."""
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        metrics_file = self.metrics_dir / f'metrics_{timestamp}.json'
        
        with metrics_file.open('w') as f:
            json.dump(self._metrics_to_dict(metrics), f, default=str)
    
    def _metrics_to_dict(self, metrics: Any) -> Dict:
        """Convert metrics to dictionary."""
        if isinstance(metrics, (SystemMetrics, ProcessMetrics)):
            return {k: str(v) if isinstance(v, datetime) else v 
                   for k, v in metrics.__dict__.items()}
        return metrics

class ResourceMonitor(Monitor):
    """Monitors system resources."""
    
    def __init__(self, project_root: Optional[Path] = None,
                 interval: int = 60):
        super().__init__(project_root)
        self.interval = interval
        self.thresholds = self._load_thresholds()
    
    def _load_thresholds(self) -> Dict[str, float]:
        """Load resource thresholds from config."""
        return {
            'cpu_percent': 80.0,
            'memory_percent': 85.0,
            'disk_usage_percent': 90.0
        }
    
    def check_thresholds(self, metrics: SystemMetrics) -> List[str]:
        """Check if metrics exceed thresholds."""
        warnings = []
        
        if metrics.cpu_percent > self.thresholds['cpu_percent']:
            warnings.append(f"CPU usage ({metrics.cpu_percent}%) exceeds threshold "
                          f"({self.thresholds['cpu_percent']}%)")
        
        if metrics.memory_percent > self.thresholds['memory_percent']:
            warnings.append(f"Memory usage ({metrics.memory_percent}%) exceeds threshold "
                          f"({self.thresholds['memory_percent']}%)")
        
        if metrics.disk_usage_percent > self.thresholds['disk_usage_percent']:
            warnings.append(f"Disk usage ({metrics.disk_usage_percent}%) exceeds threshold "
                          f"({self.thresholds['disk_usage_percent']}%)")
        
        return warnings
    
    def monitor(self, duration: Optional[int] = None):
        """Start resource monitoring."""
        start_time = time.time()
        
        try:
            while True:
                metrics = self.collect_system_metrics()
                self.save_metrics(metrics)
                
                warnings = self.check_thresholds(metrics)
                for warning in warnings:
                    self.logger.warning(warning)
                
                if duration and (time.time() - start_time) >= duration:
                    break
                
                time.sleep(self.interval)
                
        except KeyboardInterrupt:
            self.logger.info("Resource monitoring stopped")

class PerformanceMonitor(Monitor):
    """Monitors application performance."""
    
    def __init__(self, project_root: Optional[Path] = None):
        super().__init__(project_root)
        self.process_map = self._discover_processes()
    
    def _discover_processes(self) -> Dict[str, int]:
        """Discover relevant processes."""
        processes = {}
        
        for proc in psutil.process_iter(['pid', 'name', 'cmdline']):
            try:
                if 'anya' in ' '.join(proc.info['cmdline'] or []).lower():
                    processes[proc.info['name']] = proc.info['pid']
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                continue
        
        return processes
    
    def collect_performance_metrics(self) -> Dict[str, ProcessMetrics]:
        """Collect performance metrics for all processes."""
        metrics = {}
        
        for name, pid in self.process_map.items():
            try:
                metrics[name] = self.collect_process_metrics(pid)
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                self.logger.warning(f"Could not collect metrics for process {name} (PID: {pid})")
                continue
        
        return metrics
    
    def analyze_performance(self, metrics: Dict[str, ProcessMetrics]) -> List[str]:
        """Analyze performance metrics."""
        insights = []
        
        for name, process_metrics in metrics.items():
            if process_metrics.cpu_percent > 50:
                insights.append(f"High CPU usage ({process_metrics.cpu_percent}%) "
                              f"detected in process {name}")
            
            if process_metrics.memory_percent > 50:
                insights.append(f"High memory usage ({process_metrics.memory_percent}%) "
                              f"detected in process {name}")
            
            if process_metrics.connections > 1000:
                insights.append(f"High number of connections ({process_metrics.connections}) "
                              f"detected in process {name}")
        
        return insights

def start_monitoring(monitor_type: str, project_root: Optional[Path] = None,
                    duration: Optional[int] = None, interval: int = 60):
    """Start a specific type of monitoring."""
    monitors = {
        'resource': lambda: ResourceMonitor(project_root, interval),
        'performance': lambda: PerformanceMonitor(project_root)
    }
    
    if monitor_type not in monitors:
        raise ValueError(f"Unknown monitor type: {monitor_type}")
    
    monitor = monitors[monitor_type]()
    
    if monitor_type == 'resource':
        monitor.monitor(duration)
    else:
        while True:
            metrics = monitor.collect_performance_metrics()
            monitor.save_metrics(metrics)
            insights = monitor.analyze_performance(metrics)
            
            for insight in insights:
                monitor.logger.info(insight)
            
            if duration and (time.time() - monitor.start_time) >= duration:
                break
            
            time.sleep(interval)
