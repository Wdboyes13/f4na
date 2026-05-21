#include <ast.h>
#include "tokenizer.h"

Expr* parse_expr(Parser* p);
Expr* parse_logor(Parser* p);
Expr* parse_logand(Parser* p);
Expr* parse_bitor(Parser* p);
Expr* parse_bitxor(Parser* p);
Expr* parse_bitand(Parser* p);
Expr* parse_equ(Parser* p);
Expr* parse_comp(Parser* p);
Expr* parse_shift(Parser* p);
Expr* parse_add(Parser* p);
Expr* parse_term(Parser* p);
Expr* parse_unary(Parser* p);
Expr* parse_power(Parser* p);
Expr* parse_primary(Parser* p);
Expr* parse_literal(Parser* p);

Expr* parse_expr(Parser* p) { return parse_logor(p); }

Expr* parse_logor(Parser* p) {
    auto left = parse_logand(p);
    while (p->at(TK_LOGOR)) {
        auto op = p->advance().tk;
        auto right = parse_logand(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }
    return left;
}

Expr* parse_logand(Parser* p) {
    auto left = parse_bitor(p);
    while (p->at(TK_LOGAND)) {
        auto op = p->advance().tk;
        auto right = parse_bitor(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }
    return left;
}

Expr* parse_bitor(Parser* p) {
    auto left = parse_bitxor(p);
    while (p->at(TK_BITOR)) {
        auto op = p->advance().tk;
        auto right = parse_bitxor(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }

    return left;
}

Expr* parse_bitxor(Parser* p) {
    auto left = parse_bitand(p);
    while (p->at(TK_BITXOR)) {
        auto op = p->advance().tk;
        auto right = parse_bitand(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }

    return left;
}

Expr* parse_bitand(Parser* p) {
    auto left = parse_equ(p);
    while (p->at(TK_BITAND)) {
        auto op = p->advance().tk;
        auto right = parse_equ(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }

    return left;
}

Expr* parse_equ(Parser* p) {
    auto left = parse_comp(p);
    while (p->at(TK_EQ) || p->at(TK_NEQ)) {
        auto op = p->advance().tk;
        auto right = parse_comp(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }
    return left;
}

Expr* parse_comp(Parser* p) {
    auto left = parse_shift(p);
    while (p->at(TK_GT) || p->at(TK_LT) || p->at(TK_GE) || p->at(TK_LE)) {
        auto op = p->advance().tk;
        auto right = parse_shift(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }
    return left;
}

Expr* parse_shift(Parser* p) {
    auto left = parse_add(p);
    while (p->at(TK_SHL) || p->at(TK_SHR)) {
        auto op = p->advance().tk;
        auto right = parse_add(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }

    return left;
}

Expr* parse_add(Parser* p) {
    auto left = parse_term(p);
    while (p->at(TK_MIN) || p->at(TK_ADD)) {
        auto op = p->advance().tk;
        auto right = parse_term(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }
    return left;
}

Expr* parse_term(Parser* p) {
    auto left = parse_unary(p);
    while (p->at(TK_MUL) || p->at(TK_DIV) || p->at(TK_MOD)) {
        auto op = p->advance().tk;
        auto right = parse_unary(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }
    return left;
}

Expr* parse_unary(Parser* p) {
    if (p->at(TK_NOT)) {
        auto op = p->advance().tk;
        return new Expr(UnaryExpr{ op, parse_unary(p) });
    }

    if (p->at(TK_ADD) || p->at(TK_MIN)) {
        auto op = p->advance().tk;
        return new Expr(UnaryExpr{ op, parse_unary(p) });
    }

    if (p->at(TK_BNOT)) {
        auto op = p->advance().tk;
        return new Expr(UnaryExpr{ op, parse_unary(p) });
    }

    return parse_power(p);
}

Expr* parse_power(Parser* p) {
    auto left = parse_primary(p);
    if (p->at(TK_PWR)) {
        auto op = p->advance().tk;
        auto right = parse_unary(p);

        left = new Expr(BinaryExpr{ op, left, right });
    }
    return left;
}

Expr* parse_primary(Parser* p) {
    if (p->at(TK_LPAREN)) {
        p->advance();
        auto expr = parse_expr(p);

        p->expect(TK_RPAREN);
        return expr;
    }

    if (p->at(TK_IDENT)) {
        auto ident = std::get<std::string>(p->advance().val.data);
        if (p->at(TK_LPAREN)) {
            p->advance();
            std::vector<Expr*> args;
            while (!p->at(TK_RPAREN)) {
                args.push_back(parse_expr(p));

                if (p->at(TK_COMMA)) {
                    p->advance();
                }
            }
            p->advance();
            return new Expr(CallExpr{ ident, args });
        } else {
            return new Expr(IdentExpr{ ident });
        }
    }
    return parse_literal(p);
}

Expr* parse_literal(Parser* p) {
    if (p->at(TK_INT) || p->at(TK_FLOAT) ||
        p->at(TK_STRING) || p->at(TK_BOOL)) {
        return new Expr(LiteralExpr{ p->advance().val });
    }
    throw std::runtime_error(
        "unexpected token in expression: " + token_name(p->peek().tk));
}