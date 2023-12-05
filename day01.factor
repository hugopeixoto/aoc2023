USING: arrays command-line io.encodings.utf8 io.files kernel
math math.parser namespaces prettyprint sequences splitting
strings tools.time unicode ;

IN: part1
IN: part2

! returns the index counting from the end
: last-subseq-index ( seq subseq -- i/f )
  reverse swap reverse swap
  subseq-index
  ;

: part1 ( fn -- n )
  utf8 file-lines
  [ [ digit? ] filter ] map
  [ dup first swap last 2array >string string>number ] map
  sum
  ;

: part2number ( str -- num )
  "one" "1" replace
  "two" "2" replace
  "three" "3" replace
  "four" "4" replace
  "five" "5" replace
  "six" "6" replace
  "seven" "7" replace
  "eight" "8" replace
  "nine" "9" replace
  string>number
  ;

: p2words ( -- words )
  {
    "1" "2" "3" "4" "5" "6" "7" "8" "9"
    "one" "two" "three" "four" "five" "six" "seven" "eight" "nine"
  }
  ;

: part2first ( str -- num )
  p2words
  [ 2dup subseq-index swap 2array ] map
  [ first ] filter
  [ first ] minimum-by
  nip
  last
  part2number
  ;

: part2last ( str -- num )
  p2words
  [ 2dup last-subseq-index swap 2array ] map
  [ first ] filter
  [ first ] minimum-by
  nip
  last
  part2number
  ;

: part2digits ( str -- str )
  [ part2first 10 * ] [ part2last ] bi +
  ;

: part2 ( fn -- n )
  utf8 file-lines
  [ part2digits ] map
  sum
  ;

command-line get last
[ part1 . ] [ part2 . ] bi
