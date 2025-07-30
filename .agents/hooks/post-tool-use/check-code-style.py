#!/usr/bin/env python3
# /// script
# requires-python = ">=3.8"
# ///

"""
Post-tool-use hook to automatically format code files.
Supports multiple languages and provides informative output.
"""

import json
import os
import subprocess
import sys
from pathlib import Path


def check_command_exists(command):
    """Check if a command is available in PATH."""
    try:
        subprocess.run([command, "--version"], capture_output=True, check=True)
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        return False


def format_rust_file(file_path):
    """Format a Rust file using cargo fmt."""
    if not check_command_exists("cargo"):
        print("⚠️  cargo not found, skipping Rust formatting", file=sys.stderr)
        return True

    # Run cargo fmt on the entire repository
    result = subprocess.run(["cargo", "fmt", "-q"], capture_output=True)

    if result.returncode == 0:
        print(f"✅ Formatted Rust code with cargo fmt", file=sys.stderr)
        return True
    else:
        print(f"❌ Failed to format Rust code with cargo fmt", file=sys.stderr)
        if result.stderr:
            print(f"   Error: {result.stderr.decode()}", file=sys.stderr)
        return False


def format_python_file(file_path):
    """Format a Python file using ruff."""
    # Try different ways to run ruff
    ruff_commands = [
        ["ruff", "format"],  # If ruff is installed globally
        ["uvx", "ruff", "format"],  # Using uvx
        ["uv", "run", "ruff", "format"],  # Using uv run
    ]

    for cmd_prefix in ruff_commands:
        try:
            # First check if we can run ruff
            check_cmd = cmd_prefix[:-1] + ["--version"]
            check_result = subprocess.run(check_cmd, capture_output=True)

            if check_result.returncode == 0:
                # Check if formatting is needed
                check_format_cmd = cmd_prefix + ["--check", file_path]
                check_result = subprocess.run(check_format_cmd, capture_output=True)

                if check_result.returncode == 0:
                    # Already formatted
                    return True

                # Format the file
                format_cmd = cmd_prefix + [file_path]
                result = subprocess.run(format_cmd, capture_output=True)

                if result.returncode == 0:
                    print(f"✅ Formatted {file_path} with ruff", file=sys.stderr)
                    return True
                else:
                    print(f"⚠️  Failed to format {file_path} with ruff", file=sys.stderr)
                    if result.stderr:
                        print(f"   Error: {result.stderr.decode()}", file=sys.stderr)
                    # Don't block on formatting errors
                    return True
        except (subprocess.CalledProcessError, FileNotFoundError):
            continue

    # No ruff available, skip silently
    return True


def format_json_file(file_path):
    """Format a JSON file using jq if available."""
    if not check_command_exists("jq"):
        return True

    try:
        # Read and format JSON
        with open(file_path, "r") as f:
            content = f.read()

        # Validate and format with jq
        result = subprocess.run(
            ["jq", ".", "-"], input=content.encode(), capture_output=True
        )

        if result.returncode == 0:
            # Write formatted content back
            with open(file_path, "w") as f:
                f.write(result.stdout.decode())
            print(f"✅ Formatted {file_path} with jq", file=sys.stderr)
            return True
    except Exception:
        # Don't block on JSON formatting errors
        return True

    return True


def main():
    """Main hook entry point."""
    tool_call = json.load(sys.stdin)
    file_path = tool_call.get("tool_input", {}).get("file_path", "")

    if not file_path or not os.path.exists(file_path):
        return

    # Skip formatting for certain files
    skip_patterns = [
        "/target/",
        "/node_modules/",
        "/.git/",
        "/vendor/",
        ".min.",
        "-lock.",
    ]

    if any(pattern in file_path for pattern in skip_patterns):
        return

    # Format based on file extension
    path = Path(file_path)
    ext = path.suffix.lower()

    success = True

    if ext == ".rs":
        success = format_rust_file(file_path)
    elif ext == ".py":
        success = format_python_file(file_path)
    elif ext == ".json":
        success = format_json_file(file_path)
    # Add more formatters as needed

    # Exit with error if formatting failed (this will block the operation)
    if not success:
        sys.exit(2)


if __name__ == "__main__":
    main()
