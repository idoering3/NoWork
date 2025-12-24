export interface Task {
  id: number;
  name: string;
  dueDate?: string | null;
  createdAt: string;
  completed: boolean;
  completedAt?: string | null;
  tags?: Tag[] | null;
}
export interface Tag {
  name: string;
  color: 'default' | 'outline' | 'danger' | 'blue'
}