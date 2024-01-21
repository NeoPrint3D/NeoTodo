import type { Todo } from "@prisma/client";

export interface TodoInfo {
  todos: Todo[];
  newTodo: Omit<Todo, "id" | "created_at" | "completed">;
  editTodo: Todo;
  showCompleted: boolean;
  showActive: boolean;
}

export const useTodoInfo = () => {
  return useState<TodoInfo>("todos", () => ({
    todos: [],
    newTodo: Object.assign({}, defaultTodo),
    editTodo: defaultTodo,
    showCompleted: true,
    showActive: true,
  }));
};
