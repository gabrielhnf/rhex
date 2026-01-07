//User must be able to assign shortcuts to implemented functions.
//Ex.:
//create_window!(Generic {})
//
//impl Generic {
//  fn do_something(){...}
//}
//
//let g = Generic::new()
//g.assign('c', do_something) //'c' key event will trigger do_something
//
//
//
//The callback function must be implemented by the window and should not be a trait of Window
//therefore, the EventHandler must have a generic type of the parent.
