export interface Todo {
  id: string;
  title: string;
  description: string;
  created_at: Date;
  due_date: Date | null;
  completed: boolean;
}
