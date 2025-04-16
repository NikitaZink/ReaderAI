# Процесс разработки ReaderAI

## Структура веток

- `main` - стабильная версия для продакшена
- `develop` - основная ветка для разработки
- `feature/*` - ветки для разработки новых функций
- `release/*` - ветки для подготовки релизов
- `hotfix/*` - ветки для срочных исправлений

## Процесс разработки

1. **Создание новой функции**
   ```bash
   git checkout develop
   git checkout -b feature/название-функции
   ```

2. **Разработка**
   - Работайте в своей feature-ветке
   - Регулярно делайте коммиты
   - Используйте осмысленные сообщения коммитов

3. **Завершение разработки**
   ```bash
   git checkout develop
   git merge feature/название-функции
   git branch -d feature/название-функции
   ```

4. **Подготовка релиза**
   ```bash
   git checkout -b release/версия
   # Тестирование и исправление багов
   git checkout main
   git merge release/версия
   git tag -a vверсия -m "Версия версия"
   git checkout develop
   git merge release/версия
   git branch -d release/версия
   ```

5. **Срочные исправления**
   ```bash
   git checkout main
   git checkout -b hotfix/описание-исправления
   # Исправление
   git checkout main
   git merge hotfix/описание-исправления
   git tag -a vверсия -m "Исправление: описание"
   git checkout develop
   git merge hotfix/описание-исправления
   git branch -d hotfix/описание-исправления
   ```

## Правила именования коммитов

- feat: новая функция
- fix: исправление бага
- docs: изменения в документации
- style: форматирование, отступы и т.д.
- refactor: рефакторинг кода
- test: добавление или изменение тестов
- chore: обновление зависимостей, настройка сборки и т.д.

Пример: `feat(auth): добавлена базовая аутентификация` 