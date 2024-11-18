Están entre <> los elementos no terminales, sin ellos, los terminales.

Entre los terminales, algunos están declarados, como digit, y otros no, puesto que
aprovecho las palabras claves.


# Identifiers

identifier -> <identifier_start> <identifier_continue>
identifier_start -> <letter>
identifier_continue -> <letter> | <other>
letter -> a..zA..z
other -> _ | <digit>
digit -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9

regex : "^\p{XID_Start}\p{XID_Continue}*"


# Expressions

expression -> <arithmetic>
            | <logic>
            | string


arithmetic -> <arithmetic> + <term>
            | <artihmetic> - <term>
            | <term>
term -> <term> * <factor>
        | <term> / <factor>
        | <term> % <factor>
        | <factor>
factor -> <variable> ** <arithmetic>
        | <variable>
variable -> number | <identifier> | <function_call>


logic -> ! <logic>
        | <logic> or <proposition>
        | <logic> and <proposition>
        | <proposition>
        | boolean
proposition -> <proposition> > <variable>
            | <proposition> < <variable>
            | <proposition> >= <variable>
            | <proposition> <= <variable>
            | <proposition> != <variable>
            | <proposition> == <variable>

For parentheses in arithmetic and logic expressions, separate what's within them from what's outside them, and parse separately with the given rules.


# Statements

statement -> <assignment> ;
            | <function_call> ;

assignment -> <declaration> = <expression>
            | <declaration> = <tuple>
            | <declaration> = <array>

declaration -> let mut <identifier> : <type(s)>
            | let mut <identifier>
            | let <identifier> : <type(s)>
            | let <identifier>
            | const <identifier> : <type(s)>
type(s) -> (type, type, ...)
        | [type; u32]
        | type

tuple -> (<variable>, ...)
        | (string, ...)

array -> [<variable>, ...]
        | [string, ...]
        | [i32; i32]

array_use -> <identifier>[u32]
tuple_use -> <identifier>.[u32]

# Conditionals
conditional -> if <logic> {<body>}
            | if <logic> {<body>} else {<body>}
            | if <logic> {<body>} else <conditional>
body -> <expression> | <statement> | <conditional> | <loop>

# Loops
loop -> <usual_loop> | <for_loop>
usual_loop -> loop {<body>}
for_loop -> for <identifier> in <array> {<body>}
        | for <identifier> in (number..number) {<body>}
        | for <identifier> in (number..=number) {<body>}

# Functions
function -> fn <identifier> (<declaration>, ...) {<body>}
function_call -> <identifier>(<argument>, ...)

