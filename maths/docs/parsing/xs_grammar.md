# XS Grammar

Syntax for production rules is:

$A \to B\alpha$

The following represent non terminal symbols:
- uppercase letters

The following represent terminal symbols:
- lowercase words/letters
- greek symbols
- [keywords](./xs_keywords.md)

## 1. Program

### 1.1. Literals

$\text{NUM} \rightarrow \text{INT | FLT}$

$\text{VCT} \rightarrow \texttt{vector(}\text{NUM}\texttt{, }\text{NUM}\texttt{, }\text{NUM}\texttt{)}$

### 1.2. Top Level Statements

$\text{X} \rightarrow \text{I X | P}$

$\text{P} \rightarrow \text{RL P | FN P | V}_\text{top}\text{ P | }\epsilon$

where

$\color{gray} \text{X} := \text{XS script}$

$\color{gray} \text{I} := \text{Include statement}$

$\color{gray} \text{P} := \text{Program}$

$\color{gray} \text{RL} := \text{Rule def}$

$\color{gray} \text{FN} := \text{Function def}$

$\color{gray} \text{V}_\text{top} := \text{Top level variable definition}$

### 1.3. Prelude

This consists of all the constants and functions described in

1. [XS Constant Reference](https://ugc.aoe2.rocks/general/xs/constants/)
2. [XS Function Reference](https://ugc.aoe2.rocks/general/xs/functions/)

## 2. Statement
$\text{S} \rightarrow \text{V}_\text{decl}\text{ | V}_\text{def}\text{ | V}_\text{asgn}\text{ | IE | W | F | SC | R | BR | CO | LBL | GT | DBG | BRPT | DP | DM}$

$\bar{\text{S}} \rightarrow \text{S }\bar{\text{S}}\text{ | }\epsilon$

$\text{B} \rightarrow \texttt{\{}\bar{\text{ S }}\texttt{\}}$

$\text{BS} \rightarrow \text{B | S}$

where

$\color{gray} \text{S} := \text{Statement}$

$\color{gray} \bar{\text{S}} := \text{Statements}$

$\color{gray} \text{B} := \text{Body}$

$\color{gray} \text{BS} := \text{Body or statement}$

$\color{gray} \text{V}_\text{decl} := \text{Variable declaration}$

$\color{gray} \text{V}_\text{def} := \text{Variable definition}$

$\color{gray} \text{V}_\text{asgn} := \text{Variable Assignment}$

$\color{gray} \text{IE} := \text{If (Else) statement}$

$\color{gray} \text{W} := \text{While loop}$

$\color{gray} \text{F} := \text{For loop}$

$\color{gray} \text{SC} := \text{Switch case}$

$\color{gray} \text{R} := \text{Return statement}$

$\color{gray} \text{BR} := \text{Break statement}$

$\color{gray} \text{CO} := \text{Continue statement}$

$\color{gray} \text{BRPT} := \text{Breakpoint}$

### 2.1. Top Level Var Def

$\text{V}_\text{top} \rightarrow \texttt{extern } \text{V}_\text{top}\text{ | }\texttt{const }\text{V}_\text{top}\text{ | }\texttt{static }\text{V}_\text{top}$

$\text{V}_\text{top} \rightarrow \text{DTYPE ID}\texttt{ = }\text{LIT}\texttt{;}$

[//]: # (todo: double check if top level var defs can have exprs)

$\text{DTYPE} \rightarrow \texttt{int | float | bool | string | vect}$

$\text{LIT} \rightarrow \text{INT | FLT | STR | VCT | BOOL}$

Note: XS currently has bugs with defining top level strings and vectors

where

$\color{gray}\text{DTYPE} := \text{Datatype}$

$\color{gray}\text{ID} := \text{Identifier}$

$\color{gray}\text{LIT} := \text{Literal}$

### 2.2. Var Decl

$\text{V}_\text{decl} \rightarrow \texttt{static }\text{V}_\text{decl}$

$\text{V}_\text{decl} \rightarrow \text{DTYPE ID}\texttt{;}$

### 2.3. Var Def

$\text{V}_\text{def} \rightarrow \texttt{const }\text{V}_\text{def}$

$\text{V}_\text{def} \rightarrow \text{DTYPE ID}\texttt{ = }\text{E}\texttt{;}$

### 2.4. Var Assign

$\text{V}_\text{asgn} \rightarrow \text{ID}\texttt{ = }\text{E}\texttt{;}$

where

$\color{gray}\text{E} := \text{Expression}$

### 2.5. If Else

$\text{IE} \rightarrow \texttt{if ( }\text{E}\texttt{ ) }\text{BS ELSE}$

$\text{ELSE} \rightarrow \texttt{else }\text{BS | } \epsilon$

where

$\color{gray}\text{ELSE} := \text{Else branch}$

### 2.6. While

$\text{W} \rightarrow \texttt{while ( }\text{E}\texttt{ ) }\text{BS}$

### 2.7. For

$\text{F} \rightarrow \texttt{for ( }\text{V}_\text{asgn} \text{ OP}_\text{rel} \text{ INT} \texttt{ ) }\text{BS}$

where

$\color{gray}\text{OP}_\text{rel} := \text{Relational operators}$

[//]: # (todo: check for all rel op use)
[//]: # (todo: check for exp in for asgn)

### 2.8. Switch

$\text{SC} \rightarrow \texttt{switch ( E ) \{ } \text{CASES} \texttt{ \}}$

$\text{CASES} \rightarrow \text{CASE CASES | DEFAULT CASES}_\text{no default}\text{ | } \epsilon$

$\text{CASES}_\text{no default} \rightarrow \text{CASE CASES}_\text{no default}\text{ | } \epsilon$

$\text{CASE} \rightarrow \texttt{case} \text{ LIT } \texttt{:} \text{ B}$

$\text{DEFAULT} \rightarrow \texttt{default :} \text{ B}$

where

$\color{gray}\text{CASES} := \text{Optional case statements with one optional default case}$

$\color{gray}\text{CASES}_\text{no default} := \text{Optional case statements only}$

$\color{gray}\text{CASE} := \text{case statement}$

$\color{gray}\text{DEFAULT} := \text{default statement}$

### 2.9. Functions

$\text{FN} \rightarrow \texttt{extern}\text{ FN | }\texttt{mutable}\text{ FN}$

$\text{FN} \rightarrow \text{RTYPE ID ( ARGS}_\text{formal}{ ) } \text{B}$

$\text{RTYPE} \rightarrow \texttt{void}\text{ | DTYPE}$

<!-- use the interleave trick, this is still LL(1) -->
$\text{ARGS}_\text{formal} \rightarrow \text{ARG | ARG, ARGS}_\text{formal}$

$\text{ARG} \rightarrow\text{DTYPE ID = LIT | } \epsilon$

where

$\color{gray}\text{RTYPE} := \text{Return type}$

$\color{gray}\text{ARGS}_\text{formal} := \text{Formal Arguments}$

$\color{gray}\text{ARG} := \text{Argument}$


### 2.10. Return

$\text{R} \rightarrow\texttt{return ( } \text{E} \texttt{ );}$

$\text{R} \rightarrow\texttt{return;}$

### 2.11. Rules

$\text{RL} \rightarrow \texttt{rule}\text{ ID RPS B}$

$\text{RPS} \rightarrow \text{RP RPS | } \epsilon$

$\text{RP} \rightarrow \text{RP}_\text{act} \text{ | } \text{RP}_\text{grp} \text{ | } \text{RP}_\text{sfq} \text{ | } \text{RP}_\text{xfq} \text{ | } \text{RP}_\text{rim} \text{ | } \text{RP}_\text{pty}$

$\text{RP}_\text{act} \rightarrow \texttt{active | inactive}$

$\text{RP}_\text{grp} \rightarrow \texttt{group} \text{ ID}$

$\text{RP}_\text{sfq} \rightarrow \texttt{minInterval} \text{ INT | }\texttt{highFrequency}$

$\text{RP}_\text{xfq} \rightarrow \texttt{maxInterval} \text{ INT}$

$\text{RP}_\text{rim} \rightarrow \texttt{runImmediately}$

$\text{RP}_\text{pty} \rightarrow \texttt{priority} \text{ INT}$

Note: only one of each parameter can be present in a rule def, this needs to be a linting time check, unfortunately its not possible to bake it into the grammar

$\color{gray}\text{RPS} := \text{Rule parameters}$

$\color{gray}\text{RP} := \text{Rule parameter}$

$\color{gray}\text{RP}_\text{act} := \text{Active state}$

$\color{gray}\text{RP}_\text{grp} := \text{Group}$

$\color{gray}\text{RP}_\text{sfq} := \text{Min frequency}$

$\color{gray}\text{RP}_\text{xfq} := \text{Max frequency}$

$\color{gray}\text{RP}_\text{rim} := \text{Run Immediately}$

$\color{gray}\text{RP}_\text{pty} := \text{Priority}$

### 2.12. Postfix

$\text{DP} \rightarrow \text{ID}\texttt{++}$

$\text{DM} \rightarrow \text{ID}\texttt{--}$

### 2.13. Include

$\text{I} \rightarrow \texttt{include }\text{STR}\texttt{;}$

### 2.14. Break

$\text{BR} \rightarrow \texttt{break;}$

### 2.15. Continue

$\text{CO} \rightarrow \texttt{continue;}$

### 2.16. Label Def

$\text{LBL} \rightarrow \texttt{label }\text{ID}\texttt{;}$

### 2.17. Goto

$\text{GT} \rightarrow \texttt{goto }\text{ID}\texttt{;}$

### 2.18. Function Call (Statement)

$\text{FNCS} \rightarrow \text{FNC}\texttt{;}$

### 2.19. Debug

$\text{DBG} \rightarrow \texttt{dbg }\text{ID}\texttt{;}$

Note: I don't know what this does in XS, its valid syntax though.

### 2.20. Breakpoint

$\text{BRPT} \rightarrow \texttt{breakpoint;}$

Note: This will pause XS execution. I don't know if its possible to resume execution/if this keyword is useful.

### 2.21. Class

$\text{CLS} \rightarrow \texttt{class } \text{ID} \texttt{ \{ } \text{MEM\_VARS} \texttt{ \};}$

$\text{MEM\_VARS} \rightarrow \text{DTYPE ID}\texttt{ = }\text{E}\texttt{;} \text{ MEM\_VARS | } \epsilon$

Note: I don't know how to use classes in XS, its valid syntax though. The furthest I've gotten is declaring a class variable: ${\tt ClsName
obj;}$. Initialising it or accessing member values doesn't seem possible.

### 2.22. Docstring

#### Todo

[//]: # (todo)

## 3. Expression

$\text{E7} \rightarrow \text{LIT | ID | P | FNC}$

$\text{E6} \rightarrow \text{E7 | E6 }\texttt{*}\text{ E7 | E6 }\texttt{/}\text{ E7 | E6 }\texttt{\%}\text{ E7}$

$\text{E5} \rightarrow \text{E6 | E5 }\texttt{+}\text{ E6 | E5 }\texttt{-}\text{ E6}$

$\text{E4} \rightarrow \text{E5 | E4 }\texttt{<}\text{ E5 | E4 }\texttt{>}\text{ E5 | E4 }\texttt{>=}\text{ E5 | E4 }\texttt{<=}\text{ E5}$

$\text{E3} \rightarrow \text{E4 | E3 }\texttt{==}\text{ E4 | E3 }\texttt{!=}\text{ E4}$

$\text{E2} \rightarrow \text{E3 | E2 }\texttt{\&\&}\text{ E3}$

$\text{E1} \rightarrow \text{E2 | E1 }\texttt{||}\text{ E2}$

$\text{E} \rightarrow \text{E1}$


### 3.1. Parenthesis

$\text{P} \rightarrow \text{(E)}$

### 3.2. Function Call (Expression)

$\text{FNC} \rightarrow \text{ID(ARGS}_\text{actual}\text{)}$

$\text{ARGS}_\text{actual} \rightarrow \text{ARGS}' \text{ | } \epsilon$

$\text{ARGS}' \rightarrow \text{E | E, ARGS}'$

where

$\color{gray}\text{ARGS}_\text{actual} := \text{Actual Arguments}$

## 4. The Full Grammar
