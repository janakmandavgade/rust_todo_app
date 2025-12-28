// import { useState } from "react";
// import reactLogo from "./assets/react.svg";
// import { invoke } from "@tauri-apps/api/core";
// import "./App.css";

// function App() {
//   const [greetMsg, setGreetMsg] = useState("");
//   const [name, setName] = useState("");

//   async function greet() {
//     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//     setGreetMsg(await invoke("greet", { name }));
//   }

//   // async function get_current_user() {
//   //   setName(await invoke("get_current_user"));
//   // }

//   async function get_current_user() {
//     const username = await invoke<string>("get_current_user");
//     console.log(username);
//     setName(username);
//   }

//   return (
//     <main className="container">
//     <h1>
//       TODO List
//     </h1>

//     <button
//       onClick={(e) => {
//         e.preventDefault();
//         get_current_user();
//       }}
//     >
//       Get Current User
//     </button>

//     </main>
//   );
// }

// export default App;










// import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";
// import "./App.css";

// type Todo = {
//   id: number;
//   text: string;
//   completed: boolean;
// };

// function App() {
//   const [todos, setTodos] = useState<Todo[]>([
//     { id: 1, text: "Read recommended book again", completed: true },
//     { id: 2, text: "Vacation planning", completed: false },
//     { id: 3, text: "Cook dinner", completed: false },
//     { id: 4, text: "Sign up for training", completed: false },
//   ]);

//   const [filter, setFilter] = useState<"ALL" | "ACTIVE" | "COMPLETED">("ALL");
//   const [search, setSearch] = useState("");
//   const [addClicked, setAddClicked] = useState(false);

//   const toggleTodo = (id: number) => {
//     setTodos((prev) =>
//       prev.map((t) =>
//         t.id === id ? { ...t, completed: !t.completed } : t
//       )
//     );
//   };

//   const filteredTodos = todos.filter((todo) => {
//     if (filter === "ACTIVE") return !todo.completed;
//     if (filter === "COMPLETED") return todo.completed;
//     return true;
//   }).filter(todo =>
//     todo.text.toLowerCase().includes(search.toLowerCase())
//   );

//   return (
//     <main className="app">
//       <h1 className="title">TODO LIST</h1>

//       {/* Top Controls */}
//       <div className="top-bar">
//         <div className="search-box">
//           🔍
//           <input
//             placeholder="Search note..."
//             value={search}
//             onChange={(e) => setSearch(e.target.value)}
//           />
//         </div>

//         <select
//           className="filter"
//           value={filter}
//           onChange={(e) => setFilter(e.target.value as any)}
//         >
//           <option value="ALL">ALL</option>
//           <option value="ACTIVE">ACTIVE</option>
//           <option value="COMPLETED">COMPLETED</option>
//         </select>
//       </div>

//       {/* Todo List */}
//       <div className="todo-list">
//         {filteredTodos.map((todo) => (
//           <div key={todo.id} className="todo-item">
//             <input
//               type="checkbox"
//               checked={todo.completed}
//               onChange={() => toggleTodo(todo.id)}
//             />
//             <span className={todo.completed ? "done" : ""}>
//               {todo.text}
//             </span>
//           </div>
//         ))}
//       </div>

//       {/* Floating Add Button */}
//       <button className="add-btn" onClick={() => setAddClicked(true)}>＋</button>

//       {/* if (addClicked) {
//         <h1> Yes Clicked!!!!</h1>
//       } */}

//       {addClicked && <h1>Yes Clicked!!!!</h1>}
//     </main>
//   );
// }

// export default App;







// import { useState } from "react";
// import "./App.css";

// type Todo = {
//   id: number;
//   text: string;
//   completed: boolean;
// };

// function App() {
//   const [todos, setTodos] = useState<Todo[]>([
//     { id: 1, text: "Read recommended book again", completed: true },
//     { id: 2, text: "Vacation planning", completed: false },
//     { id: 3, text: "Cook dinner", completed: false },
//     { id: 4, text: "Sign up for training", completed: false },
//   ]);

//   const [filter, setFilter] = useState<"ALL" | "ACTIVE" | "COMPLETED">("ALL");
//   const [search, setSearch] = useState("");
//   const [addClicked, setAddClicked] = useState(false);
//   const [newTodo, setNewTodo] = useState("");

//   const toggleTodo = (id: number) => {
//     setTodos((prev) =>
//       prev.map((t) => (t.id === id ? { ...t, completed: !t.completed } : t))
//     );
//   };

//   const filteredTodos = todos
//     .filter((todo) => {
//       if (filter === "ACTIVE") return !todo.completed;
//       if (filter === "COMPLETED") return todo.completed;
//       return true;
//     })
//     .filter((todo) => todo.text.toLowerCase().includes(search.toLowerCase()));

//   const addTodo = () => {
//     if (newTodo.trim() === "") return;
//     setTodos((prev) => [
//       ...prev,
//       { id: Date.now(), text: newTodo.trim(), completed: false },
//     ]);
//     setNewTodo("");
//     setAddClicked(false);
//   };

//   return (
//     <main className="app">
//     <div className={`app-content ${addClicked ? "blurred" : ""}`}>
//       <h1 className="title">TODO LIST</h1>

//       {/* Top Controls */}
//       <div className="top-bar">
//         <div className="search-box">
//           🔍
//           <input
//             placeholder="Search note..."
//             value={search}
//             onChange={(e) => setSearch(e.target.value)}
//           />
//         </div>

//         <select
//           className="filter"
//           value={filter}
//           onChange={(e) => setFilter(e.target.value as any)}
//         >
//           <option value="ALL">ALL</option>
//           <option value="ACTIVE">ACTIVE</option>
//           <option value="COMPLETED">COMPLETED</option>
//         </select>
//       </div>

//       {/* Todo List */}
//       <div className="todo-list">
//         {filteredTodos.map((todo) => (
//           <div key={todo.id} className="todo-item">
//             <input
//               type="checkbox"
//               checked={todo.completed}
//               onChange={() => toggleTodo(todo.id)}
//             />
//             <span className={todo.completed ? "done" : ""}>{todo.text}</span>
//           </div>
//         ))}
//       </div>
//     </div>

//     {/* Floating Add Button */}
//     <button className="add-btn" onClick={() => setAddClicked(true)}>
//       ＋
//     </button>

//     {/* Modal Popup */}
//     {addClicked && (
//       <div className="modal-overlay">
//         <div className="modal">
//           <h2>Add New Todo</h2>
//           <input
//             type="text"
//             placeholder="Enter todo..."
//             value={newTodo}
//             onChange={(e) => setNewTodo(e.target.value)}
//           />
//           <div className="modal-buttons">
//             <button onClick={addTodo}>Add</button>
//             <button onClick={() => setAddClicked(false)}>Cancel</button>
//           </div>
//         </div>
//       </div>
//     )}
//   </main>
//   );
// }

// export default App;


import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Todo = {
  todo_id: string;
  todo_data: string;
  completed: boolean;
  created_at: string;
};

function App() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [filter, setFilter] = useState<"ALL" | "ACTIVE" | "COMPLETED">("ALL");
  const [search, setSearch] = useState("");
  const [addClicked, setAddClicked] = useState(false);
  const [newTodo, setNewTodo] = useState("");

  // --- 1. Fetch Todos on Startup ---
  useEffect(() => {
    fetchTodos();
  }, []);

  const fetchTodos = async () => {
    try {
      // Calls your Rust get_all_todos_tauri function
      const data: Todo[] = await invoke("get_all_todos_tauri");
      setTodos(data);
    } catch (err) {
      console.error("Failed to fetch todos:", err);
    }
  };

  // --- 2. Toggle Completion (Updates DB) ---
  const toggleTodo = async (id: string) => {
    try {
      // Assuming your rust command is named set_todo_as_completed_tauri
      // and it takes a "todoId" argument
      await invoke("set_todo_as_completed_tauri", { todoId: id });
      fetchTodos(); // Refresh list from DB after update
    } catch (err) {
      console.error("Update failed:", err);
    }
  };

  // --- 3. Add Todo (Updates DB) ---
  const addTodo = async () => {
    if (newTodo.trim() === "") return;
    try {
      // Assuming your rust command is named insert_todo_tauri
      // and takes a "todoText" argument
      await invoke("insert_todo_tauri", { todoText: newTodo.trim() });
      setNewTodo("");
      setAddClicked(false);
      fetchTodos(); // Refresh list
    } catch (err) {
      console.error("Insert failed:", err);
    }
  };

  // Filter logic remains in JS for speed/responsiveness
  const filteredTodos = todos
    .filter((todo) => {
      if (filter === "ACTIVE") return !todo.completed;
      if (filter === "COMPLETED") return todo.completed;
      return true;
    })
    .filter((todo) => todo.todo_data.toLowerCase().includes(search.toLowerCase()));

  return (
    <main className="app">
      <div className={`app-content ${addClicked ? "blurred" : ""}`}>
        <h1 className="title">TODO LIST</h1>

        <div className="top-bar">
          <div className="search-box">
            🔍
            <input
              placeholder="Search note..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>

          <select
            className="filter"
            value={filter}
            onChange={(e) => setFilter(e.target.value as any)}
          >
            <option value="ALL">ALL</option>
            <option value="ACTIVE">ACTIVE</option>
            <option value="COMPLETED">COMPLETED</option>
          </select>
        </div>

        <div className="todo-list">
          {filteredTodos.map((todo) => (
            <div key={todo.todo_id} className="todo-item">
              <input
                type="checkbox"
                checked={todo.completed}
                onChange={() => toggleTodo(todo.todo_id)}
              />
              {/* Note the field change: todo.todo_data */}
              <span className={todo.completed ? "done" : ""}>{todo.todo_data}</span>
            </div>
          ))}
        </div>
      </div>

      <button className="add-btn" onClick={() => setAddClicked(true)}>＋</button>

      {addClicked && (
        <div className="modal-overlay">
          <div className="modal">
            <h2>Add New Todo</h2>
            <input
              type="text"
              placeholder="Enter todo..."
              value={newTodo}
              onChange={(e) => setNewTodo(e.target.value)}
            />
            <div className="modal-buttons">
              <button onClick={addTodo}>Add</button>
              <button onClick={() => setAddClicked(false)}>Cancel</button>
            </div>
          </div>
        </div>
      )}
    </main>
  );
}

export default App;
