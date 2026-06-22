# Desktop API Adapter

This directory is reserved for the frontend adapter around Tauri commands and desktop capabilities.

The frontend should call this layer instead of importing Tauri APIs throughout pages and components. This keeps a future Electron migration possible without rewriting the UI surface.

