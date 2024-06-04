use chumsky::prelude::*;
use crate::lang::lexer::tokens::Token;

pub fn keyword() -> impl Parser<char, Token, Error = Simple<char>> {
    let vector_ = text::keyword("vector").to(Token::Vector).padded();
    let include_ = text::keyword("include").to(Token::Include).padded();
    let switch_ = text::keyword("switch").to(Token::Switch).padded();
    let case_ = text::keyword("case").to(Token::Case).padded();
    let while_ = text::keyword("while").to(Token::While).padded();
    let break_ = text::keyword("break").to(Token::Break).padded();
    let default_ = text::keyword("default").to(Token::Default).padded();
    let rule_ = text::keyword("rule").to(Token::Rule).padded();
    let if_ = text::keyword("if").to(Token::If).padded();
    let then_ = text::keyword("then").to(Token::Then).padded();
    let else_ = text::keyword("else").to(Token::Else).padded();
    let goto_ = text::keyword("goto").to(Token::Goto).padded();
    let label_ = text::keyword("label").to(Token::Label).padded();
    let for_ = text::keyword("for").to(Token::For).padded();
    let dbg_ = text::keyword("dbg").to(Token::Dbg).padded();
    let return_ = text::keyword("return").to(Token::Return).padded();
    let void_ = text::keyword("void").to(Token::Void).padded();
    let int_ = text::keyword("int").to(Token::Int).padded();
    let float_ = text::keyword("float").to(Token::Float).padded();
    let string_ = text::keyword("string").to(Token::String).padded();
    let const_ = text::keyword("const").to(Token::Const).padded();
    let priority_ = text::keyword("priority").to(Token::Priority).padded();
    let min_interval = text::keyword("minInterval").to(Token::MinInterval).padded();
    let max_interval = text::keyword("maxInterval").to(Token::MaxInterval).padded();
    let high_frequency = text::keyword("highFrequency").to(Token::HighFrequency).padded();
    let active_ = text::keyword("active").to(Token::Active).padded();
    let inactive_ = text::keyword("inactive").to(Token::Inactive).padded();
    let group_ = text::keyword("group").to(Token::Group).padded();
    let infinite_loop_limit = text::keyword("infiniteLoopLimit").to(Token::InfiniteLoopLimit).padded();
    let infinite_recursion_limit = text::keyword("infiniteRecursionLimit").to(Token::InfiniteRecursionLimit).padded();
    let breakpoint_ = text::keyword("breakpoint").to(Token::Breakpoint).padded();
    let static_ = text::keyword("static").to(Token::Static).padded();
    let continue_ = text::keyword("continue").to(Token::Continue).padded();
    let extern_ = text::keyword("extern").to(Token::Extern).padded();
    let export_ = text::keyword("export").to(Token::Export).padded();
    let run_immediately = text::keyword("runImmediately").to(Token::RunImmediately).padded();
    let mutable_ = text::keyword("mutable").to(Token::Mutable).padded();
    let class_ = text::keyword("class").to(Token::Class).padded();

    choice([
        vector_,
        include_,
        switch_,
        case_,
        while_,
        break_,
        default_,
        rule_,
        if_,
        then_,
        else_,
        goto_,
        label_,
        for_,
        dbg_,
        return_,
        void_,
        int_,
        float_,
        string_,
        const_,
        priority_,
        min_interval,
        max_interval,
        high_frequency,
        active_,
        inactive_,
        group_,
        infinite_loop_limit,
        infinite_recursion_limit,
        breakpoint_,
        static_,
        continue_,
        extern_,
        export_,
        run_immediately,
        mutable_,
        class_,
    ])
}
