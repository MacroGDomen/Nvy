# Services Layer

This directory is reserved for application services and external adapters.

Planned examples:

- `desktopApi/` for Tauri command wrappers.
- Repository or storage-facing services.
- Metadata provider adapters.
- LLM client adapters.
- Import and export services.

Page components should call services through explicit functions rather than directly reaching into Tauri APIs or storage drivers.

