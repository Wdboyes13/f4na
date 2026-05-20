#include <ast.h>
#include <stdexcept>

Expr* parse_expr(Parser* p);

static IfStmt parse_if(Parser* p);

static IfStmt parse_if(Parser* p) {
    std::vector<CondBlock> branches;
    std::optional<Block> else_body;

    auto cond = parse_expr(p);
    if (!p->at(TK_LBRACE)) {
        throw std::runtime_error("expected '{' after if condition");
    }
    p->advance();

    std::vector<Stmt> stmts;
    while (!p->at(TK_RBRACE)) {
        extern Stmt parse_stmt(Parser*);
        stmts.push_back(parse_stmt(p));
    }

    p->advance();
    branches.push_back(CondBlock{ cond, Block{ stmts } });

    while (p->at(TK_ELIF)) {
        p->advance();
        auto elif_cond = parse_expr(p);
        if (!p->at(TK_LBRACE)) {
            throw std::runtime_error("expected '{' after elif condition");
        }

        p->advance();
        std::vector<Stmt> elif_stmts;
        while (!p->at(TK_RBRACE)) {
            extern Stmt parse_stmt(Parser*);
            elif_stmts.push_back(parse_stmt(p));
        }

        p->advance();
        branches.push_back(CondBlock{ elif_cond, Block{ elif_stmts } });
    }

    if (p->at(TK_ELSE)) {
        p->advance();
        if (!p->at(TK_LBRACE)) {
            throw std::runtime_error("expected '{' after else");
        }

        p->advance();
        std::vector<Stmt> else_stmts;
        while (!p->at(TK_RBRACE)) {
            extern Stmt parse_stmt(Parser*);
            else_stmts.push_back(parse_stmt(p));
        }

        p->advance();
        else_body = Block{ else_stmts };
    }

    return IfStmt{ branches, else_body };
}

Stmt parse_stmt(Parser* p) {
    if (p->at(TK_LET)) {
        p->advance();
        auto ident = std::get<std::string>(p->advance().val.data);
        if (p->at(TK_ASSIGN)) {
            p->advance();
        } else {
            throw std::runtime_error("expected '=' after let binding name");
        }

        auto* expr = parse_expr(p);

        p->expect(TK_EOS);
        return LetStmt{ ident, expr };

    } else if (p->at(TK_IDENT) && p->peek(1).tk == TK_ASSIGN) {
        auto ident = std::get<std::string>(p->advance().val.data);
        p->advance();

        auto* expr = parse_expr(p);
        p->expect(TK_EOS);

        return AssignStmt{ ident, expr };

    } else if (p->at(TK_RET)) {
        p->advance();
        auto* expr = parse_expr(p);
        p->expect(TK_EOS);

        return RetStmt{ expr };

    } else if (p->at(TK_IF)) {
        p->advance();
        return parse_if(p);

    } else if (p->at(TK_FN)) {
        p->advance();
        auto ident = std::get<std::string>(p->advance().val.data);

        if (!p->at(TK_LPAREN)) {
            throw std::runtime_error("expected '(' after function name");
        }
        p->advance();

        std::vector<std::string> args;
        while (!p->at(TK_RPAREN)) {
            if (p->at(TK_IDENT)) {
                args.push_back(std::get<std::string>(p->advance().val.data));
            } else {
                throw std::runtime_error("expected identifier in parameter list");
            }
            if (p->at(TK_COMMA)) {
                p->advance();
            }
        }
        p->advance();

        if (!p->at(TK_LBRACE)) {
            throw std::runtime_error("expected '{' for function body");
        }
        p->advance();

        std::vector<Stmt> stmts;
        while (!p->at(TK_RBRACE)) {
            stmts.push_back(parse_stmt(p));
        }
        p->advance();

        return FnDeclStmt{ ident, args, Block{ stmts } };

    } else {
        auto* expr = parse_expr(p);
        p->expect(TK_EOS);
        return ExprStmt{ expr };
    }
}

std::vector<Stmt> parse(std::vector<Token> tks) {
    auto psr = Parser{ tks };
    std::vector<Stmt> stmts;
    while (!psr.at_end()) {
        stmts.push_back(parse_stmt(&psr));
    }
    return stmts;
}