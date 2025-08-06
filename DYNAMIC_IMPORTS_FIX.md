# Исправление проблем с динамическим импортом

## Проблемы, которые были решены

### 1. Проблемы с путями в production build

**Проблема:** Абсолютные пути `/src/...` не работают корректно в production build.
**Решение:** Заменены на относительные пути `./...`

### 2. Отсутствие обработки ошибок

**Проблема:** При ошибке загрузки компонента приложение могло упасть.
**Решение:** Добавлена обработка ошибок с fallback компонентами.

### 3. Проблемы с tree-shaking

**Проблема:** Vite мог исключить неиспользуемые компоненты из build.
**Решение:** Настроены manual chunks в rollup конфигурации.

## Внесенные изменения

### 1. Создана утилита `src/lib/utils/dynamicImports.ts`

- Класс `DynamicImportsManager` для управления динамическими импортами
- Функция `createSafeDynamicImport` для безопасного создания динамических импортов
- Кэширование компонентов для улучшения производительности
- Обработка ошибок с retry логикой

### 2. Обновлен `SidePanel.vue`

```javascript
// Было:
const pluginComponents = import.meta.glob(
  "/src/components/ui/side-panel/side-panel-items/*.vue"
);

// Стало:
const { getComponent } = createSafeDynamicImport("./side-panel-items/*.vue");
```

### 3. Обновлен `Settings.vue`

```javascript
// Было:
const componentModules = import.meta.glob("./*.vue");

// Стало:
const { getComponent, getComponentMap } = createSafeDynamicImport("./*.vue");
```

### 4. Обновлена конфигурация Vite (`vite.config.ts`)

- Добавлены manual chunks для динамически импортируемых компонентов
- Настроена оптимизация для production build
- Добавлена поддержка esnext target

## Как использовать новую утилиту

### Базовое использование:

```javascript
import { createSafeDynamicImport } from "@/lib/utils/dynamicImports";

const { getComponent } = createSafeDynamicImport("./components/*.vue");

// Получить компонент с fallback
const MyComponent = getComponent("MyComponent", FallbackComponent);
```

### Продвинутое использование:

```javascript
const { getComponentMap } = createSafeDynamicImport("./views/*.vue");

// Создать карту компонентов, исключив определенные файлы
const componentMap = getComponentMap(["Settings", "Index"]);
```

## Рекомендации для будущего

1. **Всегда используйте относительные пути** для динамических импортов
2. **Добавляйте fallback компоненты** для критически важных частей приложения
3. **Настраивайте manual chunks** для больших компонентов
4. **Используйте кэширование** для часто используемых компонентов
5. **Добавляйте логирование** для отладки проблем с загрузкой

## Отладка проблем

Если динамические импорты все еще не работают:

1. Проверьте консоль браузера на наличие ошибок
2. Убедитесь, что пути к файлам корректны
3. Проверьте, что файлы компонентов существуют
4. Убедитесь, что компоненты экспортируются как default export
5. Проверьте настройки build в vite.config.ts

## Примеры ошибок и их решения

### Ошибка: "Module not found"

**Решение:** Проверьте правильность пути и убедитесь, что файл существует

### Ошибка: "Component is not a constructor"

**Решение:** Убедитесь, что компонент экспортируется как default export

### Ошибка: "Failed to fetch dynamically imported module"

**Решение:** Проверьте настройки CORS и убедитесь, что файлы доступны в production
