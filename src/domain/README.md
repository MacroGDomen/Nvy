# Domain Layer

This directory is reserved for framework-independent business rules.

Planned examples:

- Search query parsing.
- Tag normalization.
- Actress name normalization.
- Age and cup tag calculation.
- Validation rules that should not depend on React or Tauri.

UI components should not place business rules here by side effect. Domain modules should stay testable without a browser or desktop runtime.

