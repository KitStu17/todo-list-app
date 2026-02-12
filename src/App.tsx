import { useEffect, useState } from "react";
import TodoForm from "./components/TodoForm";
import TodoList from "./components/TodoList";
import { Todo } from "./types/todo";
import "./App.css";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

function App() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadTodos();

    // 창 닫기 버튼 -> 시스템 트레이로 숨기기
    const win = getCurrentWindow();
    win.onCloseRequested((e) => {
      e.preventDefault();
      win.hide();
    })
  }, []);

  const loadTodos = async () => {
    try {
      const result = await invoke<Todo[]>("todo_get_all");
      setTodos(result);
    } catch (error) {
      console.error("Failed to load todos:", error);
    } finally {
      setLoading(false);
    }
  }

  const handleAdd = async (todo: Todo) => {
    try {
      const saved = await invoke<Todo>("todo_add", { todo });
      setTodos((prev) => [...prev, saved]);
      setShowForm(false);
    } catch (error) {
      console.error("추가 실패:", error);
    }
  };

  const handleToggle = async (id: string) => {
    const todo = todos.find((t) => t.id === id);
    if(!todo) return;

    const updated = {...todo, completed: !todo.completed};
    try {
      await invoke<Todo>("todo_update", {id, updated});
      setTodos((prev) => 
        prev.map((t) => (t.id === id ? updated : t))
      );
    } catch (error) {
      console.error("업데이트 실패:", error);
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await invoke("todo_delete", { id });
      setTodos((prev) => prev.filter((t) => t.id !== id));
    } catch (error) {
      console.error("삭제 실패:", error);
    }
  };

  const remaining = todos.filter((t) => !t.completed).length;

  return (
    <div className="app">
      <header className="app-header">
        <h1>📅 D-Day Todo</h1>
        <div className="header-right">
          {remaining > 0 && (
            <span className="badge">{remaining}개 진행중</span>
          )}
          <button
            className="btn-primary"
            onClick={() => setShowForm((prev) => !prev)}
          >
            {showForm ? "✕ 닫기" : "+ 추가"}
          </button>
        </div>
      </header>

      {showForm && <TodoForm onAdd={handleAdd} />}

      <main>
        {loading ? (
          <div className="empty-state">불러오는 중...</div>
        ) : (
          <TodoList
            todos={todos}
            onToggle={handleToggle}
            onDelete={handleDelete}
          />
        )}
      </main>
    </div>
  );
}

export default App;