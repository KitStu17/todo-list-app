import { useState } from "react";
import { Todo } from "../types/todo";
import { v4 as uuidv4 } from "uuid";

interface Props {
    onAdd: (todo: Todo) => void;
}

const NOTIFY_DAY_OPTIONS = [0, 1, 2, 3, 7 ,14, 30];

export default function TodoForm({onAdd}: Props) {
    const [title, setTitle] = useState("");
    const [description, setDescription] = useState("");
    const [dday, setDday] = useState("");
    const [notificationTime, setNotificationTime] = useState("09:00");
    const [notifyDays, setNotifyDays] = useState<number[]>([1, 3]);

    const toggleDay = (day: number) => {
        setNotifyDays((prev) =>
            prev.includes(day) ? prev.filter((d) => d !== day) : [...prev, day].sort((a, b) => a - b));
    };

    const handleSubmit = (e: React.SubmitEvent) => {
        e.preventDefault();
        if(!title.trim() || !dday) return;

        const newTodo: Todo = {
            id: uuidv4(),
            title: title.trim(),
            description: description.trim() || undefined,
            dday,
            notificationTime,
            notifyDays,
            completed: false,
            createdAt: new Date().toISOString(),
        };

        onAdd(newTodo);

        // 폼 초기화
        setTitle("");
        setDescription("");
        setDday("");
        setNotificationTime("09:00");
        setNotifyDays([1, 3]);
    };

    return (
        <form onSubmit={handleSubmit} className="todo-form">
            <h2>새 할 일 추가</h2>

            <div className="form-group">
                <label>제목 *</label>
                <input 
                    type="text" 
                    placeholder="할 일을 입력하세요" 
                    value={title} 
                    onChange={(e) => setTitle(e.target.value)} 
                    required 
                />
            </div>

            <div className="form-group">
                <label>설명</label>
                <textarea
                    placeholder="설명을 입력하세요 (선택)"
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                    rows={2}
                />
            </div>

            <div className="form-row">
                <div className="form-group">
                    <label>D-DAY 날짜 *</label>
                    <input
                        type="date"
                        value={dday}
                        min={new Date().toISOString().split("T")[0]}
                        onChange={(e) => setDday(e.target.value)}
                        required
                    />
                </div>

                <div className="form-group">
                    <label>알림 시간 *</label>
                    <input 
                        type="time"
                        value={notificationTime}
                        onChange={(e) => setNotificationTime(e.target.value)}
                    />
                </div>

                <div className="form-group">
                    <label>알림 받을 날짜</label>
                    <div className="notify-days">
                        {NOTIFY_DAY_OPTIONS.map((day) => (
                            <label key={day} className="day-checkbox">
                                <input
                                    type="checkbox"
                                    checked={notifyDays.includes(day)}
                                    onChange={() => toggleDay(day)}
                                />
                                {day === 0 ? "D-Day" : `D-${day}`}
                            </label>
                        ))}
                    </div>
                </div>

                <button type="submit" className="btn-primary">
                    추가하기
                </button>
            </div>
        </form>
    )
}