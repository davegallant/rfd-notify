# Development checks

Before committing Python changes, run Black and Pylint on the modified files.
For this repository, CI also runs the pre-commit hooks across all files; run
`pre-commit run --show-diff-on-failure --all-files` before pushing so formatting
and lint failures are caught locally.
