### Box
  - best for types with unknown size at compile time
  - transfer ownership of large data to avoid copy
  - want to implement trait object rather than specific type

---

### Cell
  - it is NOT THREAD SAFE
  - gives multiple &T and still be able to mutate it
  - avoid getting references of the value inside the cell
  - only use the methods provided by Cell

---

### Rc
  - Rc is NOT thread safe
  - allow multiple &T without giving &mut T
  - useful if you want multiple &T and not think about whether pointer is 'alive'
  - if you want to mutate data within, use RefCell
  - Rc::clone DOES NOT DEEP COPY, it only increments the reference count

---

### RefCell
  - RefCell is NOT THREAD SAFE
  - ENFORCED AT RUNTIME
  - you'll get panic! instead of compile error
  - useful only if your code follows borrowing rules but compiler is unable to comprehend


---

### Arc
  - same as Rc but THREAD SAFE
  - much more expensive

---

### Cow
  - useful when most of the time you read (&T), 
      but sometimes you allocate and modify it (&mut T)
  - e.g. want to modify &str, but most of the time you won't