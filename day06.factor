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
  [ ":" split last parse-numbers ] map
  [ first ] keep
  last
  [ 2array ] 2map
  ;

: parse-from-file-kerning ( filename -- races )
  utf8 file-lines
  [ ":" split last [ blank? not ] filter parse-numbers first ] map
  ;

! Slow version but good enough:
! (t - x) * x
! (t x -) x *
:: calculate-distance ( td x -- d ) td first x - x * ;
: number-of-ways ( td -- n )
  dup first 1 swap [a..b)
  [ dupd calculate-distance over last > ] count
  nip
  ;

! Fast version:
! -x² + tx - d > 0
! -x² + tx - d = 0
! x = (-b ± sqrt(b² - 4ac)) / 2a
! x = (-t ± sqrt(t² - 4*1*d)) / 2*(-1)
! x = (-t ± sqrt(t² - 4d)) / -2
! x = (t ± sqrt(t² - 4d)) / 2
: next-integer ( x -- n ) 1 + floor >integer ;
: prev-integer ( x -- n ) 1 - ceiling >integer ;
:: zeros ( t d -- x y )
  t t * 4 d * - sqrt :> sq
  t sq - 2 / next-integer
  t sq + 2 / prev-integer
  ;

: number-of-ways-fast ( td -- n )
  [ first ] keep last
  zeros swap - 1 +
  ;

: part1 ( filename -- result )
  parse-from-file
  [ number-of-ways-fast ] map
  product
  ;

: part2 ( filename -- result )
  parse-from-file-kerning
  number-of-ways-fast
  ;

command-line get last
[ part1 . ] keep
part2 .
