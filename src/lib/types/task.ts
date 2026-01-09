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
  id: number;
  name: string;
  color: TagColor
}

export interface NewTag {
  name: string;
  color: TagColor
}

export type TagColor = 'default' | 'secondary' | 'outline' | 'danger' | 'blue' | 'green';