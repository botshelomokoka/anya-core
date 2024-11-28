"""
Unified pipeline management system for Anya.
Handles build, test, and deployment pipelines across all platforms.
"""

import os
import sys
import subprocess
from pathlib import Path
from typing import List, Dict, Optional, Callable
from dataclasses import dataclass
from concurrent.futures import ThreadPoolExecutor
from .utils import SystemInfo, Logger
from .project_config import ProjectConfig

@dataclass
class PipelineStep:
    """Represents a single pipeline step."""
    name: str
    command: str
    args: List[str]
    cwd: Path
    env: Dict[str, str]
    condition: Optional[Callable[[], bool]] = None
    timeout: int = 3600  # 1 hour default timeout

class Pipeline:
    """Base pipeline class."""
    
    def __init__(self, project_root: Optional[Path] = None):
        self.project_root = project_root or Path(__file__).parent.parent.parent
        self.config = ProjectConfig(self.project_root)
        self.system_info = SystemInfo()
        self.logger = Logger(self.project_root / 'logs')
        self.steps: List[PipelineStep] = []
    
    def add_step(self, step: PipelineStep):
        """Add a step to the pipeline."""
        self.steps.append(step)
    
    def run(self, parallel: bool = False) -> bool:
        """Run all pipeline steps."""
        if parallel:
            return self._run_parallel()
        return self._run_sequential()
    
    def _run_sequential(self) -> bool:
        """Run pipeline steps sequentially."""
        for step in self.steps:
            if not self._execute_step(step):
                return False
        return True
    
    def _run_parallel(self) -> bool:
        """Run pipeline steps in parallel where possible."""
        with ThreadPoolExecutor() as executor:
            futures = [
                executor.submit(self._execute_step, step)
                for step in self.steps
            ]
            results = [future.result() for future in futures]
            return all(results)
    
    def _execute_step(self, step: PipelineStep) -> bool:
        """Execute a single pipeline step."""
        if step.condition and not step.condition():
            self.logger.info(f"Skipping step {step.name} - condition not met")
            return True
        
        try:
            self.logger.info(f"Running step: {step.name}")
            result = subprocess.run(
                [step.command] + step.args,
                cwd=step.cwd,
                env={**os.environ, **step.env},
                timeout=step.timeout,
                capture_output=True,
                text=True
            )
            
            if result.returncode != 0:
                self.logger.error(f"Step {step.name} failed:")
                self.logger.error(f"stdout: {result.stdout}")
                self.logger.error(f"stderr: {result.stderr}")
                return False
            
            self.logger.info(f"Step {step.name} completed successfully")
            return True
            
        except subprocess.TimeoutExpired:
            self.logger.error(f"Step {step.name} timed out after {step.timeout} seconds")
            return False
        except Exception as e:
            self.logger.error(f"Step {step.name} failed with error: {str(e)}")
            return False

class BuildPipeline(Pipeline):
    """Manages the build process."""
    
    def __init__(self, project_root: Optional[Path] = None):
        super().__init__(project_root)
        self._setup_build_steps()
    
    def _setup_build_steps(self):
        """Configure build pipeline steps."""
        # Rust build steps
        self.add_step(PipelineStep(
            name="cargo-check",
            command="cargo",
            args=["check"],
            cwd=self.project_root,
            env={"RUST_BACKTRACE": "1"}
        ))
        
        self.add_step(PipelineStep(
            name="cargo-build",
            command="cargo",
            args=["build", "--release"],
            cwd=self.project_root,
            env={"RUST_BACKTRACE": "1"}
        ))
        
        # Node.js build steps (if applicable)
        if self.config.get_feature_flags().get('web5_support'):
            self.add_step(PipelineStep(
                name="npm-install",
                command="npm",
                args=["install"],
                cwd=self.project_root,
                env={}
            ))
            
            self.add_step(PipelineStep(
                name="npm-build",
                command="npm",
                args=["run", "build"],
                cwd=self.project_root,
                env={}
            ))

class TestPipeline(Pipeline):
    """Manages the testing process."""
    
    def __init__(self, project_root: Optional[Path] = None):
        super().__init__(project_root)
        self._setup_test_steps()
    
    def _setup_test_steps(self):
        """Configure test pipeline steps."""
        # Rust tests
        self.add_step(PipelineStep(
            name="cargo-test",
            command="cargo",
            args=["test", "--all-features"],
            cwd=self.project_root,
            env={"RUST_BACKTRACE": "1"}
        ))
        
        # Integration tests
        self.add_step(PipelineStep(
            name="integration-tests",
            command="cargo",
            args=["test", "--test", "integration"],
            cwd=self.project_root,
            env={"RUST_BACKTRACE": "1"}
        ))
        
        # Blockchain tests if enabled
        if self.config.get_feature_flags().get('bitcoin_integration'):
            self.add_step(PipelineStep(
                name="bitcoin-tests",
                command="cargo",
                args=["test", "-p", "anya-bitcoin"],
                cwd=self.project_root,
                env={"RUST_BACKTRACE": "1"}
            ))

class DeployPipeline(Pipeline):
    """Manages the deployment process."""
    
    def __init__(self, project_root: Optional[Path] = None):
        super().__init__(project_root)
        self._setup_deploy_steps()
    
    def _setup_deploy_steps(self):
        """Configure deployment pipeline steps."""
        # Build release
        self.add_step(PipelineStep(
            name="build-release",
            command="cargo",
            args=["build", "--release"],
            cwd=self.project_root,
            env={"RUST_BACKTRACE": "1"}
        ))
        
        # Database migrations
        self.add_step(PipelineStep(
            name="run-migrations",
            command="cargo",
            args=["run", "--bin", "anya-migrations"],
            cwd=self.project_root,
            env={"DATABASE_URL": self.config.env_config.get("DATABASE_URL", "")}
        ))
        
        # Service deployment
        if self.system_info.get_os() == 'linux':
            self.add_step(PipelineStep(
                name="systemd-deploy",
                command="sudo",
                args=["systemctl", "restart", "anya"],
                cwd=self.project_root,
                env={}
            ))
        elif self.system_info.get_os() == 'windows':
            self.add_step(PipelineStep(
                name="windows-service-deploy",
                command="powershell",
                args=["-File", "scripts/services/deploy_service.ps1"],
                cwd=self.project_root,
                env={}
            ))

def run_pipeline(pipeline_type: str, project_root: Optional[Path] = None,
                parallel: bool = False) -> bool:
    """Run a specific pipeline type."""
    pipelines = {
        'build': BuildPipeline,
        'test': TestPipeline,
        'deploy': DeployPipeline
    }
    
    if pipeline_type not in pipelines:
        raise ValueError(f"Unknown pipeline type: {pipeline_type}")
    
    pipeline = pipelines[pipeline_type](project_root)
    return pipeline.run(parallel)
