<template>
  <TresCanvas window-size :clear-color="colors['neutral']">
    <TresPerspectiveCamera :position="[3, 3, 3]" :look-at="[0, 0, 0]" />
    <TresPointLight :position="[5, 1, 5]" :intensity="100" />
    <Suspense>
      <BackgroundScene />
    </Suspense>
    <TresAmbientLight :intensity="1" />
  </TresCanvas>
  <main class="fixed w-full">
    <div>
      <header class="p-5 flex items-center gap-1 h-20 backdrop-blur-lg">
        <img src="/logo.png" alt="logo" class="w-12 h-12" />
        <h1 class="text-3xl font-semibold ml-3">
          Welcome, Today is {{ format(new Date(), "MMM dd, yyyy") }}
        </h1>
        <div class="flex gap-2 ml-auto" v-show="todoInfo.todos.length">
          <button
            class="btn btn-sm btn-error ml-auto"
            :class="!settings.isDeletingMany && 'btn-outline'"
            @click="
              () => {
                if (settings.isDeletingMany) {
                  deleteManyTodos();
                } else {
                  settings.isDeletingMany = true;
                  settings.showActive = true;
                  settings.showCompleted = true;
                }
              }
            "
          >
            {{ settings.isDeletingMany ? "Confirm" : "Delete Many" }}
          </button>
          <button
            class="btn btn-sm btn-primary ml-auto"
            v-if="settings.isDeletingMany"
            @click="
              () => {
                settings.isDeletingMany = false;
                settings.deletedTodos = [];
              }
            "
          >
            <Icon name="fe:close" />
          </button>
        </div>
      </header>
    </div>

    <div
      class="h-[calc(100vh-10rem)] overflow-auto rounded-xl mx-5 border border-secondary"
    >
      <div class="flex flex-col gap-5 mx-5 mt-5">
        <div
          class="rounded-lg bg-base-100/60 backdrop-blur-lg flex flex-col gap-3 p-3"
        >
          <button
            class="text-2xl btn btn-ghost justify-start flex"
            @click="todoInfo.showActive = !todoInfo.showActive"
          >
            Active Todos
            <span class="text-primary">({{ activeTodos.length }})</span>
            <Icon
              :name="todoInfo.showActive ? 'fe:arrow-up' : 'fe:arrow-down'"
            />
          </button>
          <ul class="flex flex-col gap-3" v-if="todoInfo.showActive">
            <li
              class="flex items-center gap-3 bg-base-100/80 backdrop-blur-lg rounded-lg p-3 transition-all border"
              :class="
                (todoInfo.editTodo.id === todo.id && 'border-primary') ||
                (settings.deletedTodos.includes(todo.id) && 'border-error') ||
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

              <div class="ml-auto flex gap-3" v-if="!settings.isDeletingMany">
                <button
                  class="btn btn-sm btn-info"
                  @click="todoInfo.editTodo = todo"
                >
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
                  :checked="settings.deletedTodos.includes(todo.id)"
                  @change="
                    (event) =>
                      (event.target as HTMLInputElement).checked
                        ? (settings.deletedTodos = [
                            ...settings.deletedTodos,
                            todo.id,
                          ])
                        : (settings.deletedTodos = settings.deletedTodos.filter(
                            (id) => id !== todo.id
                          ))
                  "
                />
              </div>
            </li>
            <div v-if="!activeTodos.length" class="text-center text-secondary">
              No todos yet. Add one above.
            </div>
          </ul>
        </div>

        <div
          class="rounded-lg bg-base-100/60 backdrop-blur-lg flex flex-col gap-3 p-3 mb-5"
        >
          <button
            class="text-2xl btn btn-ghost justify-start flex"
            @click="todoInfo.showCompleted = !todoInfo.showCompleted"
          >
            Completed Todos
            <span class="text-primary">({{ completedTodos.length }})</span>
            <Icon
              :name="todoInfo.showCompleted ? 'fe:arrow-up' : 'fe:arrow-down'"
            />
          </button>
          <ul class="flex flex-col gap-3" v-if="todoInfo.showCompleted">
            <li
              class="flex items-center gap-3 bg-base-100/80 backdrop-blur-lg rounded-lg p-3 transition-all"
              :class="
                todoInfo.editTodo.id === todo.id && 'border-primary border-4'
              "
              :key="todo.id"
              v-for="todo in completedTodos"
            >
              <input
                type="checkbox"
                class="mr-3 checkbox-primary checkbox"
                @change="toggleTodo(todo)"
                :checked="todo.completed"
              />

              <h2
                class="text-xl font-semibold"
                :class="todo.completed && 'text-gray-400 line-through'"
              >
                {{ todo.title }}
              </h2>

              <h3 class="text-base font-normal text-secondary">
                {{ todo.description }}
              </h3>

              <h3 class="text-base font-normal text-white" v-if="todo.due_date">
                Due:
                {{ format(new Date(todo.due_date), "dd MMM yyyy, h:mm a") }}
              </h3>

              <div class="ml-auto flex gap-3" v-if="!settings.isDeletingMany">
                <button
                  class="btn btn-sm btn-info"
                  @click="todoInfo.editTodo = todo"
                >
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
                  :checked="settings.deletedTodos.includes(todo.id)"
                  @change="
                    (event) =>
                      (event.target as HTMLInputElement).checked
                        ? (settings.deletedTodos = [
                            ...settings.deletedTodos,
                            todo.id,
                          ])
                        : (settings.deletedTodos = settings.deletedTodos.filter(
                            (id) => id !== todo.id
                          ))
                  "
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

    <div class="fixed bottom-0 w-full flex justify-center bg-neutral">
      <form class="p-4" @submit.prevent="updateOrAddTodo">
        <div class="flex gap-3">
          <input
            type="text"
            class="input w-full input-secondary border-secondary/60 shadow-md shadow-secondary/20"
            required
            placeholder="What needs to be done?"
            :value="editMode ? todoInfo.editTodo.title : todoInfo.newTodo.title"
            @input="
              (event) =>
                editMode
                  ? (todoInfo.editTodo.title = (
                      event.target as HTMLInputElement
                    ).value)
                  : (todoInfo.newTodo.title = (
                      event.target as HTMLInputElement
                    ).value)
            "
          />

          <input
            type="text"
            class="input w-full input-secondary border-secondary/60 shadow-md shadow-secondary/20"
            placeholder="Description"
            :value="
              editMode
                ? todoInfo.editTodo.description
                : todoInfo.newTodo.description
            "
            @input="
              (event) =>
                editMode
                  ? (todoInfo.editTodo.description = (
                      event.target as HTMLInputElement
                    ).value)
                  : (todoInfo.newTodo.description = (
                      event.target as HTMLInputElement
                    ).value)
            "
          />

          <VueDatePicker
            :dark="true"
            v-if="editMode"
            placeholder="Due Date"
            v-model="todoInfo.editTodo.due_date"
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
            v-model="todoInfo.newTodo.due_date"
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
            @click="todoInfo.editTodo = defaultTodo"
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
  </main>
</template>

<script setup lang="ts">
import { invoke, dialog } from "@tauri-apps/api";
import VueDatePicker from "@vuepic/vue-datepicker";
import { format } from "date-fns";

import type { Todo } from "../types/Todo";
import { colors } from "config/tailwind.config";

const settings = useSettings();

const todoInfo = useTodoInfo();

const loadTodos = async () => {
  const response = await invoke<Todo[]>("get_todos");
  todoInfo.value.todos = response;
};

// update or add todo
const updateOrAddTodo = async () => {
  if (editMode.value) {
    await invoke("update_todo", {
      id: todoInfo.value.editTodo.id,
      title: todoInfo.value.editTodo.title,
      description: todoInfo.value.editTodo.description,
      dueDate: todoInfo.value.editTodo.due_date,
      completed: todoInfo.value.editTodo.completed,
    });
  } else if (todoInfo.value.newTodo.title) {
    await invoke("add_todo", {
      ...todoInfo.value.newTodo,
      dueDate: todoInfo.value.newTodo.due_date,
    });
  } else {
  }

  loadTodos();
  todoInfo.value.editTodo = {
    id: "",
    title: "",
    description: "",
    created_at: new Date(),
    due_date: null,
    completed: false,
  };
  todoInfo.value.newTodo = {
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
    "Are you sure you want to delete these todos?"
  );
  if (!res) return;
  await invoke("delete_many_todos", { ids: settings.value.deletedTodos });
  loadTodos();
  settings.value.deletedTodos = [];
  settings.value.isDeletingMany = false;
};

const activeTodos = computed(() => {
  return todoInfo.value.todos.filter((todo) => !todo.completed);
});

const completedTodos = computed(() => {
  return todoInfo.value.todos.filter((todo) => todo.completed);
});

const editMode = computed(() => {
  return !!todoInfo.value.editTodo.id;
});

onMounted(() => {
  loadTodos();
});
</script>

<style>
.dp__theme_dark {
  --dp-primary-color: var(--color-primary);
  --dp-background-color: #00002020;
  --dp-border-color: var(--color-primary);
  --dp-menu-border-color: var(--color-secondary);
  border-radius: 16px;
  backdrop-filter: blur(10px);
}
.dp__overlay {
  backdrop-filter: blur(10px);
  border-top-left-radius: 16px;
  border-top-right-radius: 16px;
}
.dp__btn {
  border-radius: 8px;
}

.dp__pointer:hover {
  background-color: #ff79c680;
}

.dp__btn:hover {
  background-color: #ff79c680;
  color: var(--color-white);
}

.dp__inner_nav:hover {
  background-color: transparent;
  color: #fff;
}
.dp__icon {
  color: #fff;
}
</style>
