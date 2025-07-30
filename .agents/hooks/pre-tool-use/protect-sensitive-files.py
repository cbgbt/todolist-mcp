#!/usr/bin/env python3
"""
Pre-tool-use hook to prevent accidental modification of sensitive files.
"""

import json
import sys
import os

# Define sensitive paths and patterns
SENSITIVE_PATHS = (
    ".env",
    ".env.local",
    ".env.production",
    ".git/",
    "Cargo.lock",  # Should be modified via cargo commands
    ".DS_Store",
    "target/",  # Build artifacts
    "node_modules/",
    "*.pyc",
    "__pycache__/",
    ".cargo/credentials",
    ".cargo/credentials.toml",
)

# Files that should only be modified with explicit user request
PROTECTED_FILES = (
    "Cargo.toml",  # Package manifest - be careful with dependencies
    ".github/workflows/",  # CI/CD configs
    ".claude/settings.json",  # This settings file
    "deny.toml",  # Security policy
)


def is_sensitive_path(file_path):
    """Check if a file path matches any sensitive patterns."""
    # Normalize path
    normalized = os.path.normpath(file_path)

    # Check exact matches and path containment
    for sensitive in SENSITIVE_PATHS:
        if sensitive.endswith("/"):
            # Directory check
            if normalized.startswith(sensitive) or f"/{sensitive}" in normalized:
                return True, sensitive
        elif "*" in sensitive:
            # Glob pattern check
            import fnmatch

            if fnmatch.fnmatch(normalized, sensitive):
                return True, sensitive
        else:
            # File check
            if sensitive in normalized:
                return True, sensitive

    return False, None


def is_protected_file(file_path):
    """Check if a file requires extra caution."""
    normalized = os.path.normpath(file_path)

    for protected in PROTECTED_FILES:
        if protected.endswith("/"):
            if normalized.startswith(protected) or f"/{protected}" in normalized:
                return True
        elif protected in normalized:
            return True

    return False


def main():
    tool_call = json.load(sys.stdin)
    file_path = tool_call.get("tool_input", {}).get("file_path", "")

    if not file_path:
        return

    # Check sensitive paths
    is_sensitive, matched_pattern = is_sensitive_path(file_path)
    if is_sensitive:
        print(f"❌ Cannot write to sensitive file: {file_path}", file=sys.stderr)
        print(f"   Matched pattern: {matched_pattern}", file=sys.stderr)
        sys.exit(1)

    # Warn about protected files (don't block, just inform)
    if is_protected_file(file_path):
        print(f"⚠️  Modifying protected file: {file_path}", file=sys.stderr)
        print("   Please ensure changes are intentional", file=sys.stderr)


if __name__ == "__main__":
    main()
