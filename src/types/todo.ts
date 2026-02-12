export interface Todo {
    id: string;
    title: string;
    description?: string;
    dday: string;       // "YYYY-MM-DD"
    notificationTime: string; // "HH:mm"
    notifyDays: number[];  // [0, 1, 2, 7] -> d-0, d-1, d-2, d-7
    completed: boolean;
    createdAt: string;
}