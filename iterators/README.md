## iter()
  - &T
  - implement iter_mut() inside T returning U that stores state and implements Iterator using Item = & T

## iter_mut()
  - &mut T
  - implement iter_mut() inside T returning U that stores state and implements Iterator using Item = &mut T

## into_iter()
  - mostly T, sometimes &T or &mut T depending on the context.
  - implemented if you implement Iterator
  - use this when you want to 'move' instead of borrow

  