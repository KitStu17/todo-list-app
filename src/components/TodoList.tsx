import { Todo } from "../types/todo";

interface Props {
    todos: Todo[];
    onToggle: (id: string) => void;
    onDelete: (id: string) => void;
}

function calcDday(ddayStr: string): number {
    const today = new Date();
    today.setHours(0, 0, 0, 0); // 오늘 날짜의 시간 부분을 초기화
    const dday = new Date(ddayStr);
    dday.setHours(0, 0, 0, 0); // D-day 날짜의 시간 부분을 초기화
    return Math.ceil((dday.getTime() - today.getTime())) / (1000 * 60 * 60 * 24);
}

function getDdayLabel(days: number): string {
  if (days === 0) return "D-Day";
  if (days > 0) return `D-${days}`;
  return `D+${Math.abs(days)}`;
}

function getDdayColor(days: number): string {
  if (days < 0) return "#999";
  if (days === 0) return "#e74c3c";
  if (days <= 3) return "#e67e22";
  return "#2ecc71";
}

export default function TodoList({todos, onToggle, onDelete}: Props) {
    const sorted = [...todos].sort((a, b) => {
        if (a.completed !== b.completed) return a.completed ? 1 : -1;
        return calcDday(a.dday) - calcDday(b.dday);
    });

    if (todos.length === 0) {
        return <div className="empty-state">
            <p>📝 할 일을 추가해보세요!</p>
        </div>
    }

    return (
    <div className="todo-list">
      {sorted.map((todo) => {
        const days = calcDday(todo.dday);
        const ddayLabel = getDdayLabel(days);
        const ddayColor = getDdayColor(days);

        return (
          <div
            key={todo.id}
            className={`todo-item ${todo.completed ? "completed" : ""}`}
          >
            <div className="todo-main">
              <div className="todo-header">
                <span
                  className="dday-badge"
                  style={{ backgroundColor: ddayColor }}
                >
                  {ddayLabel}
                </span>
                <h3 className="todo-title">{todo.title}</h3>
              </div>

              {todo.description && (
                <p className="todo-description">{todo.description}</p>
              )}

              <div className="todo-meta">
                <span>📅 {todo.dday}</span>
                <span>🔔 {todo.notificationTime}</span>
                <span>
                  알림:{" "}
                  {todo.notifyDays
                    .map((d) => (d === 0 ? "D-Day" : `D-${d}`))
                    .join(", ")}
                </span>
              </div>
            </div>

            <div className="todo-actions">
              <button
                className={todo.completed ? "btn-undo" : "btn-complete"}
                onClick={() => onToggle(todo.id)}
              >
                {todo.completed ? "↩ 되돌리기" : "✓ 완료"}
              </button>
              <button className="btn-delete" onClick={() => onDelete(todo.id)}>
                🗑
              </button>
            </div>
          </div>
        );
      })}
    </div>
  );
}