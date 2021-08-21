/**
  Option::as_ref    -->   &Option<T> into Option<&T>
                          e.g. &Option<String> --> Option<&String>
  Option::as_deref  -->   &Option<T> into Option<&U>
                          where U is target type of impl Deref for T
                          e.g. &Option<String> --> Option<&str>
**/
struct TODO;
