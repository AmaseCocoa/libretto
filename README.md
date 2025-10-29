# Libretto
A Simple and zero-dependency task runner for pyproject.toml.

## Install
```bash
uv tool install libretto
```

## Example
```toml
[tool.libretto]
venv = ".venv"

[tool.libretto.tasks]
hello = "echo 'Hello, World!'"
```