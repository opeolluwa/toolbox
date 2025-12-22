import sys
import subprocess

commit_kinds = ["feat", "chore", "refactor", "update"]

kind = input("Commit kind? ").strip()

if kind not in commit_kinds:
    print(f"Invalid commit kind. Use one of: {', '.join(sorted(commit_kinds))}")
    sys.exit(1)

commit_title = input("Commit title? ").strip()
commit_message = input("Commit message? ").strip()

message = f"""{kind}({commit_title})

{commit_message}
"""

print(message)

subprocess.run(["git", "add", "."], check=True)
subprocess.run(["git", "commit", "-m", message], check=True)
