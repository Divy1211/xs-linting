# XS Maximal Munch (WIP)

Generation of [PA](../pa) from XS.

## 1. Notation

- $M_e(E) \vdash ({\tt d}, {\tt lis})$ is a function which yields the PA addressed instructions ${\tt lis}$ which compute the value of the expression $E$ and stores the result in ${\tt d}$. An optional second argument may provide a custom name for ${\tt d}$
- $M_s(S) \vdash {\tt lis}$ is a function which yields the PA addressed instructions ${\tt lis}$ for statement $S$
- $\Delta(X)$ is a mapping of program variables to PA identifiers
- ${\tt newId} \vdash {\tt n}$ is a generator which yields a new and unique identifier
- ${\tt newAddr} \vdash {\tt l}$ is a generator which yields the next address for a PA instruction.
- ${\tt newAddr?} \vdash {\tt l}$ is a function which peeks the next address for a PA instruction without incrementing it.
- ${\tt endAddr?} \vdash {\tt l}$ is a function which yields the address of the returning/ending instruction name of the procedure/loop for which code is being generated.
- $$\begin{array}{rc}
    {\tt (xsMmCase)} & \begin{array}{c}
        \begin{array}{cc} C_1 & C_2 \end{array}
        \\ \hline
        S_1
    \end{array}
\end{array}$$

    is read as $C_1 \land C_2 \implies S_1$

## 2. MM for Expressions

### 2.1. PA Operand

$$
\begin{array}{rc}
    {\tt (xsMmSrc)} & M_e({\tt s}) \vdash ({\tt s}, \texttt{[]})
\end{array}
$$

### 2.2. Parenthesis

$$
\begin{array}{rc}
    {\tt (xsMmParen)} & \begin{array}{c}
        M_e(E) \vdash ({\tt d}, {\tt lis})
        \\ \hline
        M_e((E)) \vdash ({\tt d}, {\tt lis})
    \end{array}
\end{array}
$$

### 2.3. Function Call (Expression)

$$
\begin{array}{rc}
    {\tt (xsMmFncExpr)} & \begin{array}{c}
        \begin{array}{c}
            M_e(E_i) \vdash ({\tt d_i}, {\tt lis_i})
            \\ {\tt newAddr} \vdash {\tt l_i}
            \\ {\tt newAddr} \vdash {\tt l_{j}}
            \\ {\tt newAddr} \vdash {\tt l_{p}}
            \\ {\tt newAddr} \vdash {\tt l_r}
            \\ {\tt newId} \vdash {\tt d_r}
        \end{array}
        \\ \hline
        M_e({\tt fnName(E_1, ..., E_n)}) \vdash \left({\tt d_r}, \begin{array}{c}
            {\tt lis_1\ + ...\ + lis_n}
            \\ {\tt +\ [l_n : push\ d_n, ..., l_1 : push\ d_1]}
            \\ {\tt +\ [l_j : jmp\ fnName]}
            \\ {\tt +\ [l_p : dealloc\ n]}
            \\ {\tt +\ [ l_r : d_r \leftarrow r_{ret} ]}
        \end{array}\right)
    \end{array}
\end{array}
$$

### 2.4. Operations

$$
\begin{array}{rc}
    {\tt (xsMmOp)} & \begin{array}{c}
        \begin{array}{c}
            M_e(E_1) \vdash ({\tt d_1}, {\tt lis_1})
            \\ M_e(E_2) \vdash ({\tt d_2}, {\tt lis_2})
            \\ {\tt newId} \vdash {\tt d_3}
            \\ {\tt newAddr} \vdash {\tt l}
        \end{array}
        \\ \hline
        M_e(E_1\ {\tt op}\ E_2) \vdash ({\tt d_3}, {\tt lis_1\ +\ lis_2\ +\ } {\tt [ l : d_3 \leftarrow d_1\ op\ d_2 ]})
    \end{array}
\end{array}
$$

### 2.5. Type Casting

Whilst explicit type casting is not allowed in XS, when code is generated for an expression, there *may* be one additional step to perform an implicit type cast to the desired type for that expression. An invalid type cast will never be required in a [well-typed XS program](../static/xs_type_chk.md#11-well-typed-programs). This step is omitted from the description of the algorithm above since it would add unnecessary repetitive logic to each case and hinder clarity. In the actual algorithm, $M_e$ takes an extra argument specifying the required type $T_{req}$ and is provided with the inferred type $T_{inf}$ from the type checking phase:

$$
\begin{array}{rc}
    {\tt (xsMmImplicitCast)} & \begin{array}{c}
        \begin{array}{c}
            M_e(E) \vdash ({\tt d}, {\tt lis})
            \\ \Gamma \vdash E : T_{inf}
            \\ T_{req} \neq T_{inf}
            \\ {\tt newAddr} \vdash {\tt l}
        \end{array}
        \\ \hline
        M_e(E, T_{req}) \vdash ({\tt d}, {\tt lis\ +\ [ l : d \leftarrow tcast\ d ]})
    \end{array}
\end{array}
$$

Where ${\tt tcast}$ may be one of ${\tt icast}$, ${\tt fcast}$, or ${\tt scast}$ depending on $T_{req}$.


## 3. MM for Statements

### 3.1. Sequence

$$
\begin{array}{rc}
    {\tt (xsMmSeq)} & \begin{array}{c}
        \begin{array}{c}
            M_e(S) \vdash {\tt lis}
            \\ M_e(\bar{S}) \vdash {\tt lis'}
        \end{array}
        \\ \hline
        M_s(S \bar{S}) \vdash {\tt lis + lis'}
    \end{array}
\end{array}
$$


### 3.2. Include

Include statements don't generate code themselves, they will be resolved like C macros before code generation

### 3.3. Var Def

$$
\begin{array}{rc}
    {\tt (xsMmDef)} & \begin{array}{c}
        \begin{array}{c}
            \\ M_e(E) \vdash {({\tt d}, {\tt lis})}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s(T\ X\ =\ E{\tt ;}) \vdash {\tt lis} & \Delta \oplus (X, {\tt d})
        \end{array}
    \end{array}
\end{array}
$$

[//]: # (todo: check for def inits)

### 3.4. Var Assign

$$
\begin{array}{rc}
    {\tt (xsMmAssign)} & \begin{array}{c}
        \begin{array}{c}
            \Delta(X) \vdash {\tt d}
            \\ M_e(E, \Delta(X)) \vdash {({\tt d}, {\tt lis})}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s(X\ =\ E{\tt ;}) \vdash {\tt lis}
        \end{array}
    \end{array}
\end{array}
$$

### 3.4. If Else

$$
\begin{array}{rc}
    {\tt (xsMmIfElse)} & \begin{array}{c}
        \begin{array}{c}
            M_e(E_c) \vdash {({\tt d_c}, {\tt lis_c})}
            \\ {\tt newAddr} \vdash {\tt l_c}
            \\ M_s(\bar{S}_1) \vdash {\tt lis_1}
            \\ \color{yellow} {\tt newAddr} \vdash {\tt l_{endThen}}
            \\ {\tt newAddr?} \vdash {\tt l_{else}}
            \\ \color{yellow} M_s(\bar{S}_2) \vdash {\tt lis_2}
            \\ \color{yellow} {\tt newAddr?} \vdash {\tt l_{endIf}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt if\ (} E_c {\tt)\ \{\ } \bar{S_1} {\tt\ \}\ else\ \{\ } \bar{S_2} {\tt\ \}}) \vdash \begin{array}{c}
            {\tt lis_c}
            \\ {\tt +\ [l_c : ifn\ d_c\ goto\ l_{else}}]
            \\ {\tt +\ lis_1}
            \\ \color{yellow} {\tt +\ [l_{endThen} : goto\ l_{endIf}}]
            \\ \color{yellow} {\tt +\ lis_2}
        \end{array}
        \end{array}
    \end{array}
\end{array}
$$

Note: The instructions highlighted in yellow are not generated when an else block is not present

### 3.5. While

$$
\begin{array}{rc}
    {\tt (xsMmWhile)} & \begin{array}{c}
        \begin{array}{c}
            \\ {\tt newAddr?} \vdash {\tt l_{eval}}
            \\ M_e(E_c) \vdash {({\tt d_c}, {\tt lis_c})}
            \\ {\tt newAddr} \vdash {\tt l_c}
            \\ M_s(\bar{S}) \vdash {\tt lis}
            \\ {\tt newAddr} \vdash {\tt l_{loop}}
            \\ {\tt newAddr?} \vdash {\tt l_{end}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt while\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}}) \vdash \begin{array}{c}
            {\tt lis_c}
            \\ {\tt +\ [l_c : ifn\ d_c\ goto\ l_{end}}]
            \\ {\tt +\ lis}
            \\ {\tt +\ [l_{loop} : goto\ l_{eval}]}
        \end{array}
        \end{array}
    \end{array}
\end{array}
$$

### 3.6. For

$$
\begin{array}{rc}
    {\tt (xsMmForInc)} & \begin{array}{c}
        \begin{array}{c}
            {\tt op} \in \{{\tt <, <=}\}
            \\ \Delta(X) \vdash {\tt d_1}
            \\ M_e(E_1, \Delta(X)) \vdash {({\tt d_1}, {\tt lis_1})}
            \\ {\tt newAddr?} \vdash {\tt l_{eval}}
            \\ M_e(E_2) \vdash {({\tt d_2}, {\tt lis_2})}
            \\ {\tt newAddr} \vdash {\tt l_{cmp}}
            \\ {\tt newId} \vdash {\tt d_{cmp}}
            \\ {\tt newAddr} \vdash {\tt l_{br}}
            \\ M_s(\bar{S}) \vdash {\tt lis}
            \\ {\tt newAddr} \vdash {\tt l_{inc}}
            \\ {\tt newAddr} \vdash {\tt l_{loop}}
            \\ {\tt newAddr?} \vdash {\tt l_{end}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt for\ (}X\ =\ E_1{\tt ;}\ {\tt op}\ E_2 {\tt)\ \{\ } \bar{S} {\tt\ \}}) \vdash \begin{array}{c}
            {\tt lis_1\ +\ lis_2}
            \\ {\tt +\ [l_{cmp} : d_{cmp} \leftarrow d_1\ op\ d_2]}
            \\ {\tt +\ [l_{br} : ifn\ d_{cmp}\ goto\ l_{end}]}
            \\ {\tt +\ lis}
            \\ {\tt +\ [l_{inc} : d_1 \leftarrow d_1 + 1]}
            \\ {\tt +\ [l_{loop} : goto\ l_{eval}]}
        \end{array}
        \end{array}
    \end{array}
\end{array}
$$


$$
\begin{array}{rc}
    {\tt (xsMmForDec)} & \begin{array}{c}
        \begin{array}{c}
            {\tt op} \in \{{\tt >, >=}\}
            \\ \Delta(X) \vdash {\tt d_1}
            \\ M_e(E_1, \Delta(X)) \vdash {({\tt d_1}, {\tt lis_1})}
            \\ {\tt newAddr?} \vdash {\tt l_{eval}}
            \\ M_e(E_2) \vdash {({\tt d_2}, {\tt lis_2})}
            \\ {\tt newAddr} \vdash {\tt l_{cmp}}
            \\ {\tt newId} \vdash {\tt d_{cmp}}
            \\ {\tt newAddr} \vdash {\tt l_{br}}
            \\ M_s(\bar{S}) \vdash {\tt lis}
            \\ {\tt newAddr} \vdash {\tt l_{inc}}
            \\ {\tt newAddr} \vdash {\tt l_{loop}}
            \\ {\tt newAddr?} \vdash {\tt l_{end}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt for\ (}X\ =\ E_1{\tt ;}\ {\tt op}\ E_2 {\tt)\ \{\ } \bar{S} {\tt\ \}}) \vdash \begin{array}{c}
            {\tt lis_1\ +\ lis_2}
            \\ {\tt +\ [l_{cmp} : d_{cmp} \leftarrow d_1\ op\ d_2]}
            \\ {\tt +\ [l_{br} : ifn\ d_{c1}\ goto\ l_{end}]}
            \\ {\tt +\ lis}
            \\ {\tt +\ [l_{inc} : d_1 \leftarrow d_1 - 1]}
            \\ {\tt +\ [l_{loop} : goto\ l_{eval}]}
        \end{array}
        \end{array}
    \end{array}
\end{array}
$$

### 3.7. Switch

$$
\begin{array}{rc}
    {\tt (xsMmSwitch)} & \begin{array}{c}
        \begin{array}{cccc}
            & M_e(E_c) \vdash {({\tt d_c}, {\tt lis_c})} &
            \\ & M_e(E_i) \vdash {({\tt d_i}, {\tt lis_{eval_i}})} & {\tt newAddr} \vdash {\tt l_{cmp_i}} & {\tt newAddr} \vdash {\tt l_{br_i}}
            \\ & {\tt newAddr} \vdash {\tt l_{br_{default}}}
            \\ {\tt newAddr?} \vdash {\tt l_{start_i}} &  M_s(\bar{S}_i) \vdash {\tt lis_i} & {\tt newAddr} \vdash {\tt l_{end_i}}
            \\ {\tt newAddr?} \vdash {\tt l_{default}} & \color{yellow} M_s(\bar{S}_{default}) \vdash {\tt lis_{default}}
            \\ & {\tt newAddr?} \vdash {\tt l_{end}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt switch\ (} E_c {\tt)\ \{\ } {\tt case\ } E_1 {\tt\ :\ \{\ } \bar{S_1} {\tt\ \}} {\tt\ ...\ case\ } E_n {\tt\ :\ \{\ } \bar{S_n} {\tt\ \}} {\tt\ default\ :\ \{\ } \bar{S_d} {\tt\ \}} {\tt\ \}}) \vdash \begin{array}{c}
            {\tt lis_c}
            \\ {\tt +\ lis_{eval_1}\ +\ [l_{cmp_1} : d_{cmp_1} \leftarrow d_c\ !=\ d_1,\ l_{br_1} : ifn\ d_{cmp_1}\ goto\ l_{start_1}]}
            \\ ...
            \\ {\tt +\ lis_{eval_n}\ +\ [l_{cmp_n} : d_{cmp_n} \leftarrow d_c\ !=\ d_n,\ l_{br_n} : ifn\ d_{cmp_n}\ goto\ l_{start_n}]}
            \\ {\tt +\ [l_{br_{default}} : goto\ l_{default}]}
            \\ {\tt lis_1 + [l_{end_1} : goto\ l_{end}]}
            \\ ...
            \\ {\tt lis_n + [l_{end_n} : goto\ l_{end}]}
            \\ \color{yellow} {\tt +\ lis_{default}}
        \end{array}
        \end{array}
    \end{array}
\end{array}
$$

Note: The instructions highlighted in yellow are not generated when a default block is not present. In that case, ${\tt l_
{default}}$ is the same label as ${\tt l_{end}}$ 

### 3.8 Break, Continue, Break Point, Debug

Note: An ${\tt endAddr?}$ call will yield the ${\tt l_{end}}$ of the most recently entered for/while/switch/function body. It will never be invoked outside a proper context (outside a switch/for/while/function body) in a [well-typed XS program](../static/xs_type_chk.md#11-well-typed-programs)

### 3.9. Function Definition


### 3.10. Rule Definitions


### 3.11. Postfix


### 3.12. Label, Goto


### 3.13. Function Call (Statement)


### 3.14. Class Definition
