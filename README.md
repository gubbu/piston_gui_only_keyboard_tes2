# piston_gui_only_keyboard_tes2
## modules and demo of their function in fn main():
+ rendering text
+ adding and using the data structure "VerticalMenu", that allows for:
  + scrolling (with two keyboard keys) through a list of elements
  + select elements, that throw generic identifying Messages of type T, when selected (in demo).
  + change app state after evaluating Message <T>
  
  the demo shows the standard gui counter example: a updated label shows a changeabble number, if a button is pressed by the user and one of the following mathmatical operation is conducted on it:
 + add 1
 + substract 1
 + multiply with itseld(square), wrapping u128
 + reset to 0 (*=0)
