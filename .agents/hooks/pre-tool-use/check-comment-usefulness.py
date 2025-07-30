#!/usr/bin/env python3
# /// script
# requires-python = ">=3.8"
# ///

"""
Post-tool-use hook to remind agents about useful vs useless comments.
Lists all comments found in modified Rust/Python files and reminds the agent
to ensure comments explain "why" rather than "what".
"""

import json
import re
import sys
from pathlib import Path


def extract_comments_from_content(content, language):
    """Extract comments from content based on language."""
    comments = []
    lines = content.split("\n")

    for line_num, line in enumerate(lines, 1):
        if language == "rust":
            # Match // comments that are either at start of line or after word boundary
            # Exclude /// and //! doc comments
            match = re.search(r"(?:^|\b)\s*(//(?![/!]))(.*)$", line)
            if match:
                comment_text = match.group(1) + match.group(2)
                comments.append((line_num, comment_text.strip()))

        elif language == "python":
            # Match # comments that are either at start of line or after word boundary
            match = re.search(r"(?:^|\b)\s*(#.*)$", line)
            if match:
                comment_text = match.group(1)
                comments.append((line_num, comment_text.strip()))

    return comments


def get_edited_content(tool_call):
    """Extract the content that was written/edited."""
    tool_name = tool_call.get("tool_name", "")
    tool_input = tool_call.get("tool_input", {})

    if tool_name == "Write":
        return tool_input.get("content", "")

    elif tool_name == "Edit":
        return tool_input.get("new_string", "")

    elif tool_name == "MultiEdit":
        # Concatenate all new_strings from edits
        edits = tool_input.get("edits", [])
        contents = []
        for edit in edits:
            new_string = edit.get("new_string", "")
            if new_string:
                contents.append(new_string)
        return "\n".join(contents)

    return ""


def main():
    """Main hook entry point."""
    tool_call = json.load(sys.stdin)
    tool_name = tool_call.get("tool_name", "")
    file_path = tool_call.get("tool_input", {}).get("file_path", "")

    # Only check for Write, Edit, and MultiEdit tools
    if tool_name not in ["Write", "Edit", "MultiEdit"]:
        return

    if not file_path:
        return

    # Only check Rust and Python files
    path = Path(file_path)
    ext = path.suffix.lower()

    language = None
    if ext == ".rs":
        language = "rust"
    elif ext == ".py":
        language = "python"
    else:
        return

    # Get the content that was written/edited
    content = get_edited_content(tool_call)
    if not content:
        return

    # Extract comments from the new content
    comments = extract_comments_from_content(content, language)

    if not comments:
        return

    # Print reminder about comment usefulness
    print("\n📝 Comment Review Reminder", file=sys.stderr)
    print("=" * 50, file=sys.stderr)
    print(
        f"Found {len(comments)} comment(s) in the modified content of {file_path}:",
        file=sys.stderr,
    )
    print("", file=sys.stderr)

    for line_num, comment_text in comments:
        print(f"  Line {line_num}: {comment_text}", file=sys.stderr)

    print("\n💡 Remember:", file=sys.stderr)
    print(
        "  • USEFUL comments explain WHY code exists or does something surprising",
        file=sys.stderr,
    )
    print(
        "  • USELESS comments explain WHAT the code is doing (when it's obvious)",
        file=sys.stderr,
    )
    print("  • Only add comments when:", file=sys.stderr)
    print("    - Code is doing something non-obvious or surprising", file=sys.stderr)
    print("    - The complexity cannot be reduced through refactoring", file=sys.stderr)
    print("    - Domain-specific context is needed", file=sys.stderr)
    print(
        "\n⚠️  Please review the comments above and ensure they add value!",
        file=sys.stderr,
    )
    print("=" * 50, file=sys.stderr)
    print("", file=sys.stderr)


if __name__ == "__main__":
    main()
