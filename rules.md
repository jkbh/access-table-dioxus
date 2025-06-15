# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Development
- `dx serve` - Start development server (default desktop platform)
- `dx serve --platform web` - Run for web platform
- `dx serve --platform desktop` - Run for desktop platform
- `dx serve --platform mobile` - Run for mobile platform

### Build
- `cargo build` - Build the project
- `cargo check` - Check for compilation errors

### Testing
- `cargo test` - Run tests

## Architecture

This is a Dioxus application that demonstrates a data table with interactive cell selection functionality. The project uses Dioxus 0.7.0-alpha.0 with multi-platform support (web, desktop, mobile).

### Key Components
- **App**: Root component that creates mock users and renders the main table
- **Table**: Main table component with complex selection state management
- **SelectionStore**: Manages row, column, and cell selections using HashSet collections
- **RangeSelectionState**: Handles drag-to-select functionality with mouse interactions
- **GroupCell/RowHeader/GroupHeader**: Individual cell and header components with selection logic

### State Management
The application uses Dioxus signals and context providers for state management:
- `SelectionStore` tracks selected rows, columns, and individual cells
- `RangeSelectionState` manages drag selection with IndexRect and IndexRange structs
- Selection state is shared between components via context

### Data Model
- **User**: Contains id, name, properties (IndexMap), and groups (HashMap<String, bool>)
- **Row**: Wrapper around User with additional id field
- **GroupColumn/PropertyColumn**: Column definitions that access user data

### Styling
Uses Tailwind CSS with automatic compilation via Dioxus 0.7+ (no manual setup required). CSS classes are applied conditionally based on selection state.

### Features
- Multi-platform support (web/desktop/mobile via Cargo features)
- Interactive table with row/column/cell selection
- Drag-to-select rectangular cell ranges
- Mock data generation using the `fake` crate
- Sticky headers for better UX with large datasets