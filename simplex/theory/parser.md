# Parser

## Grammar

Tokens: 
- (LParen), (RParen)
- (Number)
- (Variable)
- (Function)
- (Plus) , (Minus), (Times), (Divide)
- (Eq), (Leq), (Geq), (LT), (GT)
- (EOL)

```
<Program> ::= <Objective> (EOL) <Constraints>
<Objective> ::= (Function={"max", "min"}) (LParen="{") <Expression> (RParen="}")
<Constraints> ::= (Function="st")  (LParen="{") <Constraint> (RParen="}")

<CMP> :== (Eq) | (Leq) | (Geq) | (LT) | (GT)
<Term> :== <Number> | <Variable> | <Number> <Variable> | (LParen) <Expression> (RParen)

<Statement> :== <Term> <Statement_1>
<Statement_1> :== (Times) <Statement_1>
<Statement_1> :== (Divide) <Statement_1>
<Statement_1> :== e

<Expression> :== <Statement> <Expression_1>
<Expression_1> :== (Plus) <Expression_1>
<Expression_1> :== (Minus) <Expression_1>
<Expression_1> :== e


<Constraint> :== <Expression> <CMP> <Number> <Constraint_1>
<Constraint_1> :== (EOL) <Expression> <CMP> <Number> <Constraint_1>
<Constraint_1> :== (EOL)
```

```
Program: "max" | EOF
Objective: "max" | EOL
Constraints: "st" | EOF

CMP: {Eq, Leq, Geq, LT, GT} | <Number>
TERM: {<Number>, <Variable>, LParen} | {<ArithOp>, <CMP>, RParen }

STATEMENT: {<Number>, <Variable>, LParen} | {Plus, Minus, <CMP>, RParen}
STATEMENT_1: {Times, Divide, e} | {Plus, Minus, <CMP>, RParen}

EXPRESSION: {LParen, <Number>, <Variable>} | {<CMP>, RParen}
EXPRESSION_1: {Minus, Plus} | {<CMP>, RParen}

CONSTRAINT: {LParen, <Number>, <Variable>} | {RParen}
CONSTRAINT_1: {eol} | {RParen}
```

```
PROGRAM -> OBJECTIVE eol CONSTRAINTS
OBJECTIVE -> max { EXPRESSION }
CONSTRAINTS -> st { CONSTRAINT }

CONSTRAINT -> EXPRESSION cmp number CONSTRAINTA
CONSTRAINTA -> eol EXPRESSION cmp number CONSTRAINTA
CONSTRAINTA -> eol

EXPRESSION -> EXPRESSIONP EXPRESSIONM
EXPRESSIONP -> TERM EXPRESSIONPA
EXPRESSIONPA -> * EXPRESSIONPA 
EXPRESSIONPA -> / EXPRESSIONPA 
EXPRESSIONPA -> epsilon

EXPRESSIONM -> + EXPRESSIONM
EXPRESSIONM -> - EXPRESSIONM
EXPRESSIONM -> epsilon

TERM ->  number 
TERM ->  variable 
TERM ->  number variable 
TERM ->  ( EXPRESSION ) 
TERM ->  [ EXPRESSION ] 
```

```
Non-terminals: PROGRAM OBJECTIVE CONSTRAINTS EXPRESSION CONSTRAINT CONSTRAINTA EXPRESSIONP EXPRESSIONM TERM EXPRESSIONPA
Terminals: eol max { } st cmp number * / + - variable ( ) [ ]
EPS = EXPRESSIONM EXPRESSIONPA
FIRST[PROGRAM] = max
FIRST[OBJECTIVE] = max
FIRST[CONSTRAINTS] = st
FIRST[EXPRESSION] = number variable ( [
FIRST[CONSTRAINT] = number variable ( [
FIRST[CONSTRAINTA] = eol
FIRST[EXPRESSIONP] = number variable ( [
FIRST[EXPRESSIONM] = + -
FIRST[TERM] = number variable ( [
FIRST[EXPRESSIONPA] = * /
FOLLOW[PROGRAM] =
FOLLOW[OBJECTIVE] = eol
FOLLOW[CONSTRAINTS] =
FOLLOW[EXPRESSION] = } cmp ) ]
FOLLOW[CONSTRAINT] = }
FOLLOW[CONSTRAINTA] = }
FOLLOW[EXPRESSIONP] = } cmp + - ) ]
FOLLOW[EXPRESSIONM] = } cmp ) ]
FOLLOW[TERM] = } cmp * / + - ) ]
FOLLOW[EXPRESSIONPA] = } cmp + - ) ]
PREDICT:
PROGRAM ->  OBJECTIVE eol CONSTRAINTS : max
OBJECTIVE ->  max { EXPRESSION } : max
CONSTRAINTS ->  st { CONSTRAINT } : st
CONSTRAINT ->  EXPRESSION cmp number CONSTRAINTA : number variable ( [
CONSTRAINTA ->  eol EXPRESSION cmp number CONSTRAINTA : eol
CONSTRAINTA ->  eol : eol
EXPRESSION ->  EXPRESSIONP EXPRESSIONM : number variable ( [
EXPRESSIONP ->  TERM EXPRESSIONPA : number variable ( [
EXPRESSIONPA ->  * EXPRESSIONPA : *
EXPRESSIONPA ->  / EXPRESSIONPA : /
EXPRESSIONPA -> epsilon : } cmp + - ) ]
EXPRESSIONM ->  + EXPRESSIONM : +
EXPRESSIONM ->  - EXPRESSIONM : -
EXPRESSIONM -> epsilon : } cmp ) ]
TERM ->  number : number
TERM ->  variable : variable
TERM ->  number variable : number
TERM ->  ( EXPRESSION ) : (
TERM ->  [ EXPRESSION ] : [
```
