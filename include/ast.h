#pragma once
#include <optional>
#include <stdexcept>
#include <string>
#include <variant>
#include "tokenizer.h"
#include "value.h"

struct Stmt;
struct Expr;

struct BinaryExpr {
    TokenType op;
    Expr *left, *right;
};

struct UnaryExpr {
    TokenType op;
    Expr* arg;
};

struct CallExpr {
    std::string ident;
    std::vector<Expr*> args;
};

struct IdentExpr {
    std::string ident;
};

struct LiteralExpr {
    Value val;
};

struct Expr : std::variant<BinaryExpr, UnaryExpr, CallExpr, IdentExpr, LiteralExpr> {
    using variant::variant;
};

struct Block {
    std::vector<Stmt> stmts;
};

struct LetStmt {
    std::string name;
    Expr* expr;
};

struct AssignStmt {
    std::string name;
    Expr* expr;
};

struct RetStmt {
    Expr* expr;
};

struct CondBlock {
    Expr* cond;
    Block body;
};

struct IfStmt {
    std::vector<CondBlock> branches;
    std::optional<Block> else_body;
};

struct WhileStmt {
    CondBlock blk;
};

struct FnDeclStmt {
    std::string name;
    std::vector<std::string> args;
    Block body;
};

struct ExprStmt {
    Expr* expr;
};

struct Stmt : std::variant<LetStmt, AssignStmt, RetStmt, IfStmt, FnDeclStmt, ExprStmt, WhileStmt> {
    using variant::variant;
};

struct Parser {
    std::vector<Token> tokens;
    size_t pos = 0;

    bool at_end() {
        return pos >= tokens.size() || tokens[pos].tk == TK_EOF;
    }

    Token peek(int n = 0) {
        if (pos + n >= tokens.size()) {
            return { TK_EOF };
        }
        return tokens[pos + n];
    }

    Token advance() {
        if (pos >= tokens.size()) {
            return { TK_EOF };
        }
        return tokens[pos++];
    }

    bool at(TokenType t) {
        return peek().tk == t;
    }

    Token expect(TokenType t) {
        if (!at(t)) {
            throw std::runtime_error("expected " + token_name(t) + " got " + token_name(peek().tk));
        }
        return advance();
    }
};

std::vector<Stmt> parse(std::vector<Token> tks);