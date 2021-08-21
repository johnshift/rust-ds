/**
If a type implements the Iterator trait, it is an iterator - it can be iterated over.
If a type implements the IntoIterator trait, it (probably) isn't an iterator - it just has a method into_iter that will get you an iterator over it.

iter()  ->  &T
        ->  implement iter_mut() inside T returning U that stores state
            and implements Iterator using Item = & T

iter_mut()  -> &mut T
            ->  implement iter_mut() inside T returning U that stores state
                and implements Iterator using Item = &mut T

into_iter() ->  mostly T, sometimes &T or &mut T depending on the context.
            ->  implemented if you implement Iterator
            ->  use this when you want to 'move' instead of borrow


**/
struct TODO;

// note that what's iterated are containers
// but what's returned are usize
