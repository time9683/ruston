# Identifiers
regex: "^\p{XID_Start}\p{XID_Continue}*"

# Expressions
expression -> <arithmetic> | <logic> | string

arithmetic -> {<term> ('+'|'-')}* <term> 
term -> {<factor> ('*'|'/') }* <factor> 
factor -> <variable> '**' <arithmetic>
variable -> number | <identifier> | <function_call>

logic -> {!} (<proposition> {('&'|'|') <proposition>}) | boolean
proposition -> {<variable> ('>' | '<' | '>=' | '<=' | '==') }+ <variable>

For parentheses in arithmetic and logic expressions, separate what's within them from what's outside them, and parse separately with the given rules.

# Statements

statement -> (<assignment> | <function_call>) ';'

assignment -> <declaration> '=' (<expression> | <tuple> | <array>)

declaration -> ('let' {'mut'}|'const') <identifier> {: <type(s)>}
type(s) -> '(' type {',' type}* ')' | '[' type ';' u32 ']' | type

tuple -> '(' (<variable> {',' <variable>}+) | (string {',' string}+) ')' 
array -> '[' (<variable> {',' <variable>}+) | (<variable> {',' <variable>}+) | (i32 ';' i32) ']'

array_use -> <identifier>[u32]
tuple_use -> <identifier>.[u32]

# Conditionals
conditional -> 'if' <logic> '{' <body> '}' {'else' ('{' <body> '}' | <conditional>)}
body ->  <expression> | <statement> | <conditional> | <loop>

# Loops
loop -> 'loop' | ('for' <identifier> 'in' (<array> | '('number'..'{'='}?number')')) '{' <body> '}'

# Functions
function -> 'fn' <identifier> '(' <declaration> {',' <declaration>}*')' '{' <body> '}'
function_call -> <identifier> '('<variable> {',' <variable>}*')'