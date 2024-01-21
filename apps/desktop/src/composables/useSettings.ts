interface Settings {
  showActive: boolean;
  showCompleted: boolean;
  isDeletingMany: boolean;
  deletedTodos: string[];
}

export const useSettings = () => {
  return useState<Settings>("settings", () => ({
    showActive: true,
    showCompleted: true,
    isDeletingMany: false,
    deletedTodos: [],
  }));
};
