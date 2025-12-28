# Todo App

A simple, cross-platform Todo application built with Tauri, React, and Rust. This app allows you to manage your daily tasks with create, read, and update operations, featuring a clean UI with search and filter capabilities.

## Features

- **Create Todos**: Add new tasks to your list
- **Read Todos**: View all your tasks with filtering options
- **Update Todos**: Mark tasks as completed
- **Search**: Find specific todos by text
- **Filter**: View all, active, or completed todos
- **Local Storage**: Data is stored locally in a SQLite database
- **Cross-Platform**: Runs on Windows, macOS, and Linux

## Tech Stack

- **Frontend**: React 18 with TypeScript
- **Backend**: Rust with Tauri 2
- **Database**: SQLite (via rusqlite)
- **Build Tool**: Vite
- **Styling**: CSS

## Prerequisites

Before running this application, make sure you have the following installed:

- [Node.js](https://nodejs.org/) (version 16 or higher)
- [Rust](https://rustup.rs/) (latest stable version)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installing Prerequisites

1. **Node.js**: Download and install from the official website
2. **Rust**: Run the following command:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. **Tauri CLI**: Install globally using npm:
   ```bash
   npm install -g @tauri-apps/cli
   ```

## Installation

1. Clone the repository:
   ```bash
   git clone [<repository-url>](https://github.com/janakmandavgade/rust_todo_app.git)
   cd todo
   ```

2. Install frontend dependencies:
   ```bash
   npm install
   ```

3. Run the application in development mode:
   ```bash
   npm run tauri dev
   ```

The application will open in a new window. The first run will create the necessary database file in your app data directory.

## Usage

### Adding a Todo
- Click the "+" button in the bottom right corner
- Enter your task in the modal that appears
- Click "Add" to save the todo

### Marking as Completed
- Check the checkbox next to any todo to mark it as completed
- Use the filter dropdown to view only active or completed todos

### Searching Todos
- Use the search box at the top to find todos by text
- The search is case-insensitive and works on partial matches

### Filtering Todos
- Use the dropdown next to the search box to filter by:
  - **ALL**: Show all todos
  - **ACTIVE**: Show only incomplete todos
  - **COMPLETED**: Show only completed todos

## Building for Production

To build the application for distribution:

```bash
npm run tauri build
```

This will create platform-specific installers in the `src-tauri/target/release/bundle/` directory.

## Project Structure

```
todo/
├── src/                    # React frontend
│   ├── App.tsx            # Main React component
│   ├── App.css            # Styles
│   └── main.tsx           # React entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Tauri commands and setup
│   │   ├── main.rs        # Application entry point
│   │   └── db/            # Database module
│   │       ├── connection.rs    # Database connection and setup
│   │       ├── crud_methods.rs  # CRUD operations
│   │       └── mod.rs
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── package.json           # Node.js dependencies and scripts
└── README.md              # This file
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) - Framework for building desktop apps with web technologies
- [React](https://reactjs.org/) - JavaScript library for building user interfaces
- [SQLite](https://www.sqlite.org/) - Self-contained, file-based SQL database
