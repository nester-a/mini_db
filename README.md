# Задание 4: Мини база данных (комбинация всех тем)

**Цель:** Комплексное применение всех пройденных концепций.

**Задача:** Создать простую in-memory базу данных для хранения пользователей.

## Требования

**Структуры:**

```rust
struct User {
    id: u32,
    username: String,
    email: String,
    active: bool,
}

struct UserDatabase {
    users: HashMap<u32, User>,
    next_id: u32,
}
```

**Методы для `UserDatase`:**

1. `new() -> Self`
2. `add_user(&mut self, username: String, email: String) -> Result<u32, String>`
* Проверяет уникальность `username` и `email`
* Возвращает ошибку если пользователь существует
3. `get_user(&self, id: u32) -> Option<&User>`
4. `delete_user(&mut self, id: u32) -> Result<(), String>`
5. `get_all_users(&self) -> Vec<&User>`

**Дополнительно:**
* Организуйте код в модули: `db`, `models`, `errors`
* Создайте собственный тип ошибки `DatabaseError` с помощью `thiserror` или вручную