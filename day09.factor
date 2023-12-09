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

: parse-from-file ( filename -- races )
  utf8 file-lines
  [ parse-numbers ] map
  ;

: next-in-sequence ( ns -- n )
  dup [ zero? ] all?
  [ drop 0 ]
  [ dup 2 clump [ [ last ] keep first - ] map next-in-sequence swap last + ]
  if
  ;

: previous-in-sequence ( ns -- n )
  dup [ zero? ] all?
  [ drop 0 ]
  [ dup 2 clump [ [ last ] keep first - ] map previous-in-sequence swap first swap - ]
  if
  ;

: part1 ( filename -- result )
  parse-from-file
  [ next-in-sequence ] map-sum
  ;

: part2 ( filename -- result )
  parse-from-file
  [ previous-in-sequence ] map-sum
  ;

command-line get last
[ part1 . ] keep
part2 .
