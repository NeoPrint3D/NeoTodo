import { Todo } from "../types/Todo";
export const defaultTodo: Todo = {
  id: "",
  title: "",
  description: "",
  due_date: null,
  created_at: new Date(),
  completed: false,
};
