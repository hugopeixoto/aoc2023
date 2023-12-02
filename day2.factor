USING: arrays command-line io.encodings.utf8 io.files kernel
math math.parser namespaces prettyprint sequences splitting
strings tools.time unicode ;

IN: part1
IN: part2

TUPLE: game-draw
  { red integer initial: 0 }
  { green integer initial: 0 }
  { blue integer initial: 0 }
  ;

: make-draw ( r g b -- draw )
  game-draw new
  swap >>blue
  swap >>green
  swap >>red
  ;

: parse-draw ( seq -- game-draw )
  "," split
  [ [ blank? ] trim ] map
  [ " " split ] map
  game-draw new
  swap
  [ {
      { [ dup last "red" = ] [ first string>number >>red ] }
      { [ dup last "green" = ] [ first string>number >>green ] }
      { [ dup last "blue" = ] [ first string>number >>blue ] }
    } cond
  ] each
  ;

TUPLE: game
  { id integer }
  { draws sequence }
  ;

: parse-game ( str -- p2r )
  ":" split
  [ last ";" split [ parse-draw ] map ] keep
  first
  " " split last string>number
  game new
  swap >>id
  swap >>draws
  ;

: is-valid-draw? ( draw -- t/f )
  [ red>> 12 <= ] keep
  [ green>> 13 <= ] keep
  blue>> 14 <=
  and and
  ;

: is-valid-game? ( game -- t/f )
  draws>>
  [ is-valid-draw? ] all?
  ;

: part1 ( filename -- result )
  utf8 file-lines
  [ parse-game ] map
  [ is-valid-game? ] filter
  [ id>> ] map
  sum
  ;

: minimum-draw ( game -- draw )
  draws>>
  0 0 0 make-draw
  [ ! ( acc elt -- elt )
    [ red>> [ red>> ] dip max ] 2keep
    [ green>> [ green>> ] dip max ] 2keep
    blue>> [ blue>> ] dip max
    make-draw
  ] reduce
  ;

: draw-power ( draw -- power )
  [ red>> ] keep
  [ green>> ] keep
  blue>>
  * *
  ;

: part2 ( filename -- result )
  utf8 file-lines
  [ parse-game ] map
  [ minimum-draw ] map
  [ draw-power ] map
  sum
  ;

command-line get last
[ part1 . ] keep
part2 .
