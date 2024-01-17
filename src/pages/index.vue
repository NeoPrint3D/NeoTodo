<template>
  <div>
    <div class="p-5 flex items-center gap-1 h-20">
      <img src="/logo.png" alt="logo" class="w-12 h-12" />
      <h1 class="text-3xl font-semibold ml-3">
        Welcome, Today is {{ format(new Date(), "MMM dd, yyyy") }}
      </h1>
      <div class="flex gap-2 ml-auto">
        <button
          class="btn btn-sm btn-error ml-auto"
          :class="!isDeletingMany && 'btn-outline'"
          @click="
            () => {
              if (isDeletingMany) {
                deleteManyTodos();
              } else {
                isDeletingMany = true;
                showActive = true;
                showCompleted = true;
              }
            }
          "
        >
          {{ isDeletingMany ? "Confirm" : "Delete Many" }}
        </button>
        <button
          class="btn btn-sm btn-primary ml-auto"
          v-if="isDeletingMany"
          @click="
            () => {
              isDeletingMany = false;
              deletedTodos = [];
            }
          "
        >
          <Icon name="fe:close" />
        </button>
      </div>
    </div>
    <div
      class="h-[calc(100vh-10rem)] overflow-auto bg-base-100 rounded-xl mx-5 border border-primary"
    >
      <div class="flex flex-col gap-5 mx-5 mt-5">
        <div class="rounded-lg bg-neutral flex flex-col gap-3 p-3">
          <button
            class="text-2xl btn btn-ghost justify-start flex"
            @click="showActive = !showActive"
          >
            Active Todos
            <span class="text-primary">({{ activeTodos.length }})</span>
            <Icon :name="showActive ? 'fe:arrow-up' : 'fe:arrow-down'" />
          </button>
          <ul class="flex flex-col gap-3" v-if="showActive">
            <li
              class="flex items-center gap-3 bg-base-100 rounded-lg p-3 transition-all border"
              :class="
                (editTodo.id === todo.id && 'border-primary') ||
                (deletedTodos.includes(todo.id) && 'border-error') ||
                'border-white/40'
              "
              :key="todo.id"
              v-for="todo in activeTodos"
            >
              <input
                type="checkbox"
                class="mr-3 checkbox-primary checkbox"
                @change="toggleTodo(todo)"
                :checked="todo.completed"
              />
              <h2 class="text-xl font-semibold">
                {{ todo.title }}
              </h2>

              <h3 class="text-base font-normal text-secondary">
                {{ todo.description }}
              </h3>

              <h3 class="text-base font-normal text-white" v-if="todo.due_date">
                Due:
                {{ format(new Date(todo.due_date), "dd MMM yyyy, h:mm a") }}
              </h3>

              <div class="ml-auto flex gap-3" v-if="!isDeletingMany">
                <button class="btn btn-sm btn-info" @click="editTodo = todo">
                  <Icon name="fe:edit" />
                </button>
                <button
                  class="btn btn-sm btn-error"
                  @click="deleteTodo(todo.id)"
                >
                  <Icon name="fe:trash" />
                </button>
              </div>
              <div class="ml-auto flex gap-3" v-else>
                <input
                  type="checkbox"
                  class="checkbox-error checkbox"
                  :checked="deletedTodos.includes(todo.id)"
                  @change="event=> (event.target as HTMLInputElement).checked ? deletedTodos = [...deletedTodos, todo.id] : deletedTodos = deletedTodos.filter(id => id !== todo.id)"
                />
              </div>
            </li>
            <div v-if="!activeTodos.length" class="text-center text-secondary">
              No todos yet. Add one above.
            </div>
          </ul>
        </div>

        <div class="rounded-lg bg-neutral flex flex-col gap-3 p-3 mb-5">
          <button
            class="text-2xl btn btn-ghost justify-start flex"
            @click="showCompleted = !showCompleted"
          >
            Completed Todos
            <span class="text-primary">({{ completedTodos.length }})</span>
            <Icon :name="showCompleted ? 'fe:arrow-up' : 'fe:arrow-down'" />
          </button>
          <ul class="flex flex-col gap-3" v-if="showCompleted">
            <li
              class="flex items-center gap-3 bg-base-100 rounded-lg p-3 transition-all"
              :class="editTodo.id === todo.id && 'border-primary border-4'"
              :key="todo.id"
              v-for="todo in completedTodos"
            >
              <input
                type="checkbox"
                class="mr-3 checkbox-primary checkbox"
                @change="toggleTodo(todo)"
                :checked="todo.completed"
              />
              <h2 class="text-xl font-semibold">
                {{ todo.title }}
              </h2>

              <h3 class="text-base font-normal text-secondary">
                {{ todo.description }}
              </h3>

              <h3 class="text-base font-normal text-white" v-if="todo.due_date">
                Due:
                {{ format(new Date(todo.due_date), "dd MMM yyyy, h:mm a") }}
              </h3>

              <div class="ml-auto flex gap-3" v-if="!isDeletingMany">
                <button class="btn btn-sm btn-info" @click="editTodo = todo">
                  <Icon name="fe:edit" />
                </button>
                <button
                  class="btn btn-sm btn-error"
                  @click="deleteTodo(todo.id)"
                >
                  <Icon name="fe:trash" />
                </button>
              </div>
              <div class="ml-auto flex gap-3" v-else>
                <input
                  type="checkbox"
                  class="checkbox-error checkbox"
                  :checked="deletedTodos.includes(todo.id)"
                  @change="event=> (event.target as HTMLInputElement).checked ? deletedTodos = [...deletedTodos, todo.id] : deletedTodos = deletedTodos.filter(id => id !== todo.id)"
                />
              </div>
            </li>
            <div
              v-if="!completedTodos.length"
              class="text-center text-secondary"
            >
              No todos completed yet.
            </div>
          </ul>
        </div>
      </div>
    </div>

    <div class="fixed bottom-0 bg-neutral w-full flex justify-center">
      <form class="p-4" @submit.prevent="updateOrAddTodo">
        <div class="flex gap-3">
          <input
            type="text"
            class="input w-full input-secondary border-secondary/60 shadow-md shadow-secondary/20"
            required
            placeholder="What needs to be done?"
            :value="editMode ? editTodo.title : newTodo.title"
            @input="(event) =>  editMode ? (editTodo.title = (event.target as HTMLInputElement).value) : (newTodo.title = (event.target as HTMLInputElement).value)"
          />

          <input
            type="text"
            class="input w-full input-secondary border-secondary/60 shadow-md shadow-secondary/20"
            placeholder="Description"
            :value="editMode ? editTodo.description : newTodo.description"
            @input="(event) => editMode ? (editTodo.description = (event.target as HTMLInputElement).value) : (newTodo.description = (event.target as HTMLInputElement).value)"
          />

          <VueDatePicker
            :dark="true"
            v-if="editMode"
            placeholder="Due Date"
            v-model="editTodo.due_date"
            calendar-cell-class-name="!rounded-xl"
            input-class-name="!input !input-secondary !text-sm !text-center !border-secondary/60 shadow-md shadow-secondary/20"
          >
            <template #input-icon>
              <Icon name="fe:calendar" class="ml-2 text-white" size="16" />
            </template>
            <template #action-row="{ internalModelValue, selectDate }">
              <div class="flex w-full items-center">
                <div class="ml-auto">
                  {{
                    internalModelValue
                      ? format(internalModelValue, "dd MMM yyyy, h:mm a")
                      : "No due date"
                  }}
                </div>
                <button
                  class="btn btn-primary btn-sm ml-auto"
                  type="button"
                  @click="selectDate"
                >
                  Select
                </button>
              </div>
            </template>
          </VueDatePicker>
          <VueDatePicker
            :dark="true"
            v-else
            placeholder="Due Date"
            v-model="newTodo.due_date"
            calendar-cell-class-name="!rounded-xl"
            input-class-name="!input !input-secondary !text-center !border-secondary/60 shadow-md shadow-secondary/20"
          >
            <template #input-icon>
              <Icon name="fe:calendar" class="ml-2 text-white" size="20" />
            </template>
            <template #action-row="{ internalModelValue, selectDate }">
              <div class="flex w-full items-center">
                <div class="ml-auto">
                  {{
                    internalModelValue
                      ? format(internalModelValue, "dd MMM yyyy, h:mm a")
                      : "No due date"
                  }}
                </div>
                <button
                  class="btn btn-primary btn-sm ml-auto"
                  type="button"
                  @click="selectDate"
                >
                  Select
                </button>
              </div>
            </template>
          </VueDatePicker>

          <button
            class="btn btn-error"
            type="button"
            @click="editTodo = defaultNewTodo"
            v-show="editMode"
          >
            <Icon name="fe:close" size="24" />
          </button>
          <button class="btn btn-primary">
            {{ editMode ? "Update" : "Add" }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke, dialog } from "@tauri-apps/api";
import VueDatePicker from "@vuepic/vue-datepicker";
import { format } from "date-fns";
interface Todo {
  id: string;
  title: string;
  description: string;
  created_at: Date;
  due_date: Date | null;
  completed: boolean;
}

const defaultNewTodo: Todo = {
  id: "",
  title: "",
  description: "",
  due_date: null,
  created_at: new Date(),
  completed: false,
};

const todos = ref<Todo[]>([]);
const newTodo = ref<Omit<Todo, "id" | "created_at" | "completed">>({
  title: "",
  description: "",
  due_date: null,
});

const editTodo = ref<Todo>({
  id: "",
  title: "",
  description: "",
  created_at: new Date(),
  due_date: null,
  completed: false,
});

const showActive = ref(true);
const showCompleted = ref(false);

const isDeletingMany = ref(false);
const deletedTodos = ref<string[]>([]);

const loadTodos = async () => {
  const response = await invoke<Todo[]>("get_todos");
  todos.value = response;
};

// update or add todo
const updateOrAddTodo = async () => {
  if (editMode.value) {
    await invoke("update_todo", {
      id: editTodo.value.id,
      title: editTodo.value.title,
      description: editTodo.value.description,
      due_date: editTodo.value.due_date,
    });
  } else if (newTodo.value.title) {
    console.log(newTodo.value);
    await invoke("add_todo", {
      ...newTodo.value,
      dueDate: newTodo.value.due_date,
    });
  } else {
  }

  loadTodos();
  editTodo.value = {
    id: "",
    title: "",
    description: "",
    created_at: new Date(),
    due_date: null,
    completed: false,
  };
  newTodo.value = {
    title: "",
    description: "",
    due_date: null,
  };
};

const toggleTodo = async (todo: Todo) => {
  await new Promise((resolve) => setTimeout(resolve, 250));
  await invoke("update_todo", {
    ...todo,
    dueDate: todo.due_date,
    completed: !todo.completed,
  });
  loadTodos();
};

const deleteTodo = async (id: string) => {
  const res = await dialog.confirm(
    "Are you sure you want to delete this todo?"
  );
  if (!res) return;
  await invoke("delete_todo", { id });
  loadTodos();
};

const deleteManyTodos = async () => {
  const res = await dialog.confirm(
    `Are you sure you want to delete ${deletedTodos.value.length} todos?`
  );
  if (!res) return;
  await Promise.all([
    ...deletedTodos.value.map((id) => invoke("delete_todo", { id })),
  ]);
  loadTodos();
  isDeletingMany.value = false;
  deletedTodos.value = [];
};

const activeTodos = computed(() => {
  return todos.value.filter((todo) => !todo.completed);
});

const completedTodos = computed(() => {
  return todos.value.filter((todo) => todo.completed);
});

const editMode = computed(() => {
  return !!editTodo.value.id;
});

onMounted(() => {
  loadTodos();
});
</script>

<style>
.dp__theme_dark {
  --dp-primary-color: var(--color-primary);
  --dp-background-color: var(--color-base-100);
  --dp-border-color: var(--color-primary);
  --dp-menu-border-color: var(--color-secondary);
}
</style>
