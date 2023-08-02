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

