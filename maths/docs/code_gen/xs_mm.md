# XS Maximal Munch

Generation of PA from XS.

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

<!-- todo: check for def inits -->

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
            \\ {\tt newAddr} \vdash {\tt l_{c1}}
            \\ {\tt newAddr} \vdash {\tt l_{c2}}
            \\ M_s(\bar{S}) \vdash {\tt lis}
            \\ {\tt newAddr} \vdash {\tt l_{inc}}
            \\ {\tt newAddr} \vdash {\tt l_{loop}}
            \\ {\tt newAddr?} \vdash {\tt l_{end}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt for\ (}X\ =\ E_1{\tt ;}\ {\tt op}\ E_2 {\tt)\ \{\ } \bar{S} {\tt\ \}}) \vdash \begin{array}{c}
            {\tt lis_1\ +\ lis_2}
            \\ {\tt +\ [l_{c1} : d_{c1} \leftarrow d_1\ op\ d_2]}
            \\ {\tt +\ [l_{c2} : ifn\ d_{c1}\ goto\ l_{end}]}
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
            \\ {\tt newAddr} \vdash {\tt l_{c1}}
            \\ {\tt newAddr} \vdash {\tt l_{c2}}
            \\ M_s(\bar{S}) \vdash {\tt lis}
            \\ {\tt newAddr} \vdash {\tt l_{inc}}
            \\ {\tt newAddr} \vdash {\tt l_{loop}}
            \\ {\tt newAddr?} \vdash {\tt l_{end}}
        \end{array}
        \\ \hline
        \begin{array}{cc}
            M_s({\tt for\ (}X\ =\ E_1{\tt ;}\ {\tt op}\ E_2 {\tt)\ \{\ } \bar{S} {\tt\ \}}) \vdash \begin{array}{c}
            {\tt lis_1\ +\ lis_2}
            \\ {\tt +\ [l_{c1} : d_{c1} \leftarrow d_1\ op\ d_2]}
            \\ {\tt +\ [l_{c2} : ifn\ d_{c1}\ goto\ l_{end}]}
            \\ {\tt +\ lis}
            \\ {\tt +\ [l_{inc} : d_1 \leftarrow d_1 - 1]}
            \\ {\tt +\ [l_{loop} : goto\ l_{eval}]}
        \end{array}
        \end{array}
    \end{array}
\end{array}
$$