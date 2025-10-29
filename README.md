# Libretto
A Simple task runner for pyproject.toml, written in Rust.

## Install
```bash
uv tool install libretto
```

## Configuration

Libretto can be configured in the `[tool.libretto]` section of your `pyproject.toml` file.

### `venv`

Specifies the path to the virtual environment to use when running tasks.

```toml
[tool.libretto]
venv = ".venv"
```

## Example
```toml
[tool.libretto]
venv = ".venv"

[tool.libretto.tasks]
hello = "echo 'Hello, World!'"
```

## Usage

Libretto reads tasks from the `pyproject.toml` file in the current directory. Tasks are defined in the `[tool.libretto.tasks]` section.

To run a task, use the `libretto` command followed by the task name:

```bash
libretto <task_name>
```

For example, to run the `hello` task from the example above:

```bash
libretto hello
```

This will output:

```
Hello, World!
```

### Passing Additional Arguments

You can pass additional arguments to your tasks. For example, if you have a task defined as:

```toml
[tool.libretto.tasks]
greet = "echo Hello"
```

You can run it with an additional argument:

```bash
libretto greet John
```

This will output:

```
Hello John
```

### Command Lists

You can also define a list of commands for a single task. Libretto will execute them in order.

```toml
[tool.libretto.tasks]
build = [
    "echo Building...",
    "py -m build"
]
```

Running `libretto build` will execute both commands.

### Platform-Specific Commands

You can define platform-specific commands by using a list of tables. list of available platforms is [here](https://doc.rust-lang.org/std/env/consts/constant.OS.html).

```toml
[tool.libretto.tasks]
test = [
    { cmd = "rm --rf dist", platforms = ["linux", "macos"] },
    { cmd = "rmdir /s /q dist", platforms = ["windows"] }
]
```

Libretto will only execute the command that matches the current platform.