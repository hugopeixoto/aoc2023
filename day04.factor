USING: arrays command-line io.encodings.utf8 io.files kernel
math math.parser namespaces prettyprint sequences splitting
strings tools.time unicode sets ;

IN: part1
IN: part2


: parse-numbers ( line -- numbers )
  [ blank? ] trim
  " " split
  [ string>number ] map
  [ ] filter
  ;

: parse-scratch-card ( line -- scratch-card )
  ":" split
  [ first " " split last string>number ] keep
  last
  "|" split
  [ first parse-numbers ] keep
  last parse-numbers
  3array
  ;

: parse-board-from-file ( filename -- board )
  utf8 file-lines
  [ parse-scratch-card ] map
  ;

: winner-numbers ( scratch-card -- n )
  [ second ] keep
  third
  intersect
  length
  ;

: points ( n -- m )
  dup zero?
  [ drop 0 ]
  [ 1 - 2 swap ^ ] if
  ;

: part1 ( filename -- result )
  parse-board-from-file
  [ winner-numbers points ] map-sum
  ;

TUPLE: scratch-card
  id
  points
  copies
  ;

C: <scratch-card> scratch-card

:: add-points ( points acc scratch-card -- points acc scratch-card )
  points scratch-card copies>> + :> points
  points acc scratch-card
  ;

:: pop-acc ( acc sc -- acc sc )
  sc [ acc first + ] change-copies :> sc
  0 acc remove-nth :> acc
  0 9 acc insert-nth :> acc
  acc sc
  ;

:: add-wins ( acc scratch-card -- acc )
  scratch-card points>> scratch-card copies>> <array> { 0 0 0 0 0 0 0 0 0 0 } append
  acc
  [ + ] 2map
  ;

: process-scratch-card ( points acc scratch-card -- points acc )
  pop-acc
  add-points
  add-wins
  ;

: part2 ( filename -- result )
  parse-board-from-file
  [ [ winner-numbers ] keep first swap 1 <scratch-card> ] map
  [ 0 { 0 0 0 0 0 0 0 0 0 0 } ] dip
  [ process-scratch-card ] each
  drop
  ;

command-line get last
[ part1 . ] keep
part2 .
