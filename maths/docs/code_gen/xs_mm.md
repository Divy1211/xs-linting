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
            \Delta \vdash {\tt fnName} \Downarrow (\{{\tt (id_1, L_1), ..., (id_n, L_n)}\}, \bar{S})
            \\ M_e(E_i) \vdash ({\tt d_i}, {\tt lis_i})
            \\ M_e(L_{k}) \vdash ({\tt d_{k}}, {\tt lis_{k}})
            \\ {\tt newAddr} \vdash {\tt l_i}
            \\ {\tt newAddr} \vdash {\tt l_{j}}
            \\ {\tt newAddr} \vdash {\tt l_{p}}
            \\ {\tt newAddr} \vdash {\tt l_r}
        \end{array}
        \\ \hline
        M_e({\tt fnName(E_1, ..., E_j)}) \vdash \left({\tt r_{ret}}, \begin{array}{c}
            {\tt lis_1\ + ...\ + lis_n}
            \\ {\tt +\ [l_n : push\ d_n, ..., l_1 : push\ d_1]}
            \\ {\tt +\ [l_j : jmp\ fnName]}
            \\ {\tt +\ [l_p : dealloc\ n]}
        \end{array}\right)
    \end{array}
\end{array}
$$

Note$^1$: If the provided number of arguments $j$ are less than the number of arguments that a function can accept $n$, the remaining ($k \in [j+1, n]$) parameters are filled by defaults at the call site itself. A function call with more than the acceptable number of parameters will never occur in a [well-typed XS program](../static/xs_type_chk.md#11-well-typed-programs)

Note$^2$: calls to void functions cannot be used as expressions. A void function call in an expression will never occur in a [well-typed XS program](../static/xs_type_chk.md#11-well-typed-programs)

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
    {\tt (xsMmFor)} & \begin{array}{c}
        \begin{array}{c}
            {\tt \ op} \in \{{\tt <, <=, >, >=}\}
            \\ {\tt \#} \in \{{\tt +, -}\}
            \\ \Delta(X) \vdash {\tt d_1}
            \\ M_e(E_1, \Delta(X)) \vdash {({\tt d_1}, {\tt lis_1})}
            \\ \color{aqua} {\tt newAddr?} \vdash {\tt l_{eval}}
            \\ \color{aqua} M_e(E_2) \vdash {({\tt d_2}, {\tt lis_2})}
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
            {\tt lis_1\ +\ {\color{aqua} lis_2}}
            \\ {\tt +\ [l_{cmp} : d_{cmp} \leftarrow d_1\ op\ d_2]}
            \\ {\tt +\ [l_{br} : ifn\ d_{cmp}\ goto\ l_{end}]}
            \\ {\tt +\ lis}
            \\ {\tt +\ [l_{inc} : d_1 \leftarrow d_1\ \#\ 1]}
            \\ {\tt +\ [l_{loop} : goto\ {\color{aqua} l_{eval}}]}
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
            \\ & \color{pink} M_e(E_i) \vdash {({\tt d_i}, {\tt lis_{eval_i}})} & \color{pink} {\tt newAddr} \vdash {\tt l_{cmp_i}} & \color{pink} {\tt newAddr} \vdash {\tt l_{br_i}}
            \\ & {\tt newAddr} \vdash {\tt l_{br_{default}}}
            \\ \color{aqua} {\tt newAddr?} \vdash {\tt l_{start_i}} & \color{aqua} M_s(\bar{S}_i) \vdash {\tt lis_i} & {\tt newAddr} \vdash {\tt l_{end_i}}
            \\ {\tt newAddr?} \vdash {\tt l_{default}} & \color{yellow} M_s(\bar{S}_{d}) \vdash {\tt lis_{default}}
            \\ & {\tt newAddr?} \vdash {\tt l_{end}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt switch\ (} E_c {\tt)\ \{\ } {\tt case\ } E_1 {\tt\ :\ \{\ } \bar{S_1} {\tt\ \}} {\tt\ ...\ case\ } E_n {\tt\ :\ \{\ } \bar{S_n} {\tt\ \}} {\tt\ default\ :\ \{\ } \bar{S_d} {\tt\ \}} {\tt\ \}}) \vdash \begin{array}{c}
            {\tt lis_c}
            \\ \color{pink} {\tt +\ lis_{eval_1}}
            \\ \color{pink} {\tt +\ [l_{cmp_1} : d_{cmp_1} \leftarrow d_c\ !=\ d_1]}
            \\ \color{pink} {\tt +\ [l_{br_1} : ifn\ d_{cmp_1}\ goto\ {\color{aqua} l_{start_1}}]}
            \\ ...
            \\ \color{violet} {\tt +\ lis_{eval_n}}
            \\ \color{violet} {\tt +\ [l_{cmp_n} : d_{cmp_n} \leftarrow d_c\ !=\ d_n]}
            \\ \color{violet} {\tt +\ [l_{br_n} : ifn\ d_{cmp_n}\ goto\ {\color{aqua} l_{start_n}}]}
            \\ {\tt +\ [l_{br_{default}} : goto\ l_{default}]}
            \\ {\tt {\color{aqua} lis_1} + [l_{end_1} : goto\ l_{end}]}
            \\ ...
            \\ {\tt {\color{aqua} lis_n} + [l_{end_n} : goto\ l_{end}]}
            \\ \color{yellow} {\tt +\ lis_{default}}
        \end{array}
        \end{array}
    \end{array}
\end{array}
$$

Note$^1$: The instructions highlighted in yellow are not generated when a default block is not present. In that case, ${\tt l_
{default}}$ is the same label as ${\tt l_{end}}$

Note$^2$: The instructions highlighted in pink are repeated as a group

### 3.8 Break, Return, Continue, Break Point, Debug


$$
\begin{array}{rc}
    {\tt (xsMmBr)} & \begin{array}{c}
        \begin{array}{c}
            {\tt endAddr?} \vdash {\tt l_{end}}
            \\ {\tt newAddr} \vdash {\tt l_{br}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt break;}) \vdash {\tt [l_{br} : goto\ l_{end}]}
        \end{array}
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsMmCo)} & \begin{array}{c}
        \begin{array}{c}
            {\tt endAddr?} \vdash {\tt l_{end}}
            \\ {\tt newAddr} \vdash {\tt l_{co`}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt continue;}) \vdash {\tt [l_{co} : goto\ l_{end} - 1]}
        \end{array}
    \end{array}
\end{array}
$$

Note: ${\tt l_{end} - 1}$ is the address of $l_{loop}$ which goes to the start of the loop to evaluate the loop condition again.

$$
\begin{array}{rc}
    {\tt (xsMmRet)} & \begin{array}{c}
        \begin{array}{c}
            M_e(E_r, {\tt r_{ret}}) \vdash {({\tt r_{ret}}, {\tt lis_r})}
            \\ {\tt endAddr?} \vdash {\tt l_{end}}
            \\ {\tt newAddr} \vdash {\tt l_{ret}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt return\ } {\color{yellow}(E_r)} {\tt ;}) \vdash \begin{array}{c}
                \color{yellow} {\tt lis_r}
                \\ {\tt +\ [l_{br} : goto\ l_{end}]}
        \end{array}
        \end{array}
    \end{array}
\end{array}
$$

Note$^1$: instructions highlighted in yellow are not generated for a void return.

Note$^2$: An ${\tt endAddr?}$ call will yield the ${\tt l_{end}}$ of the most recently entered for/while/switch/function body. It will never be invoked outside a proper context (outside a switch/for/while/function body) in a [well-typed XS program](../static/xs_type_chk.md#11-well-typed-programs)

$$
\begin{array}{rc}
    {\tt (xsMmBrPt)} & M_s({\tt breakpoint;}) \vdash {\tt []}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsMmDbg)} & M_s({\tt dgb\ id;}) \vdash {\tt []}
\end{array}
$$

Note: It is currently unknown what ${\tt breakpoint;}$ and ${\tt dbg\ id;}$ do, however ${\tt breakpoint;}$ seems to pause the XS Runtime in DE with no known way of resuming execution. These may be repurposed for an external debugger in the future.

### 3.9. Function Definition

$$
\begin{array}{rc}
    {\tt (xsMmFnDef)} & \begin{array}{c}
        \begin{array}{c}
            \color{lime} {\tt newAddr} \vdash {\tt l_{s_{lp}}}
            \\ \color{lime} {\tt newAddr} \vdash {\tt l_{s_{bp}}}
            \\ {\tt newAddr} \vdash {\tt l_{m_{bp}}}
            \\ \color{aqua} {\tt newAddr} \vdash {\tt l_{loca}}
            \\ \color{lime} {\tt newAddr} \vdash {\tt l_{s_{r_i}}}
            \\ \color{magenta} {\tt newAddr} \vdash {\tt l_{marg_i}}
            \\ \color{magenta} {\tt newAddr} \vdash {\tt l_{larg_i}}
            \\ M_s(\bar{S}) \vdash {\tt lis}
            \\ {\tt newAddr?} \vdash {\tt l_{end}}
            \\ \color{red} {\tt newAddr} \vdash {\tt l_{l_{r_i}}}
            \\ \color{orange} {\tt newAddr} \vdash {\tt l_{locd}}
            \\ \color{red} {\tt newAddr} \vdash {\tt l_{l_{bp}}}
            \\ \color{red} {\tt newAddr} \vdash {\tt l_{l_{lp}}}
            \\ {\tt newAddr} \vdash {\tt l_{ret}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt fnName(T_1\ id_1\ =\ E_1,\ ...,\ T_n\ id_n\ =\ E_n )\ \{\ } \bar{S} {\tt\ \}}) \vdash \begin{array}{c}
                \\ \color{lime} {\tt [l_{s_{lp}} : push\ r_{lp}]}
                \\ \color{lime} {\tt +\ [l_{s_{bp}} : push\ r_{bp}]}
                \\ {\tt +\ [l_{m_{bp}} : r_{bp} \leftarrow r_{sp}]}
                \\ \color{aqua} {\tt +\ [l_{loca} : alloc\ k]}
                \\ \color{lime} {\tt +\ [l_{s_{r_1}} : push\ r_{1}]}
                \\ \color{lime} {\tt +\ ...}
                \\ \color{lime} {\tt +\ [l_{s_{r_{26}}} : push\ r_{26}]}
                \\ \color{magenta} {\tt +\ [l_{marg_i} : r_i \leftarrow r_{bp} - (2 + i)]}
                \\ \color{magenta} {\tt +\ [l_{larg_i} : r_i \leftarrow mem[r_i]]}
                \\ {\tt +\ lis}
                \\ \color{red} {\tt +\ [l_{l_{r_{26}}} : pop\ r_{26}]}
                \\ \color{red} {\tt +\ ...}
                \\ \color{red} {\tt +\ [l_{l_{r_1}} : pop\ r_{1}]}
                \\ \color{orange} {\tt +\ [l_{locd} : dealloc\ k]}
                \\ \color{red} {\tt +\ [l_{l_{bp}} : pop\ r_{bp}]}
                \\ \color{red} {\tt +\ [l_{l_{lp}} : pop\ r_{lp}]}
                \\ {\tt +\ [l_{r} : ret]}
        \end{array}
        \\ \Delta \oplus ({\tt fnName}, (\{{\tt (id_1, L_1), ..., (id_n, L_n)}\}, \bar{S}))
        \end{array}
    \end{array}
\end{array}
$$

Note$^1$: ${\tt (2 + i)}$ is a compile-time constant, only binary operations are allowed in the RHS of PA 

Note$^2$: The instructions highlighted in green save the values of the registers to the stack so that they can be restored after the procedure finishes. The instructions highlighted in red restore these stored values before the procedure returns

Note$^3$: The instruction highlighted in teal allocates space for $k$ local variables, and the instruction highlighted in orange deallocates this space.

Note$^4$: The instructions highlighted in purple load the parameters passed to the function in registers

### 3.10. Rule Definitions

Same as [§ 3.9. Function Definition](#39-function-definition), except without the purple instructions since rules take no arguments

### 3.11. Postfix

$$
\begin{array}{rc}
    {\tt (xsMmPostfix)} & \begin{array}{c}
    {\tt newAddr} \vdash {\tt l_{pf}}
    \\ {\tt \# \in \{+, -\}}
    \\ \hline
    \\ M_s({\tt id\#\#;}) \vdash {\tt [l_{pf} : id\ \#\ 1]}
    \end{array}
\end{array}
$$


### 3.12. Label, Goto

$$
\begin{array}{rc}
    {\tt (xsMmLabel)} & \begin{array}{c}
    {\tt newAddr} \vdash {\tt l_{lbl}}
    \\ \hline
    \\ M_s({\tt label\ id;}) \vdash {\tt [l_{lbl} : id]}
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsMmGoto)} & \begin{array}{c}
    {\tt newAddr} \vdash {\tt l_{gto}}
    \\ \hline
    \\ M_s({\tt goto\ id;}) \vdash {\tt [l_{gto} : goto\ id]}
    \end{array}
\end{array}
$$

Note: An invalid ${\tt goto}$ will not occur in a [well-typed XS program](../static/xs_type_chk.md#11-well-typed-programs).

### 3.13. Function Call (Statement)

Same as [§ 2.3. Function Call (Expression)](#23-function-call-expression)

### 3.14. Class Definition

$$
\begin{array}{rc}
    {\tt (xsMmClsDef)} & M_s({\tt class\ clsName\ \{}\ T_1\ id_1\ =\ E_1;\ ...\ T_n\ id_n\ =\ E_n; {\tt\ \};}) \vdash {\tt []}
\end{array}
$$

Classes act purely as a type checking tool and do not generate any code. They cannot be used in XS anyway
