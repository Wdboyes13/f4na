#pragma once
#include <value.h>
#include <string>
#include <vector>

enum TokenType {
    TK_STRING,
    TK_FN,
    TK_RET,
    TK_IF,
    TK_ELIF,
    TK_ELSE,
    TK_LPAREN,
    TK_RPAREN,
    TK_LBRACE,
    TK_RBRACE,
    TK_LOGOR,
    TK_LOGAND,
    TK_EQ,
    TK_NEQ,
    TK_GT,
    TK_LT,
    TK_GE,
    TK_LE,
    TK_ADD,
    TK_MIN,
    TK_MUL,
    TK_DIV,
    TK_MOD,
    TK_NOT,
    TK_PWR,
    TK_COMMA,
    TK_FLOAT,
    TK_INT,
    TK_BOOL,
    TK_EOS,
    TK_IDENT,
    TK_LET,
    TK_ASSIGN,
    TK_EOF
};

inline std::string token_name(TokenType t) {
    switch (t) {
        case TK_STRING:
            return "STRING";
        case TK_FN:
            return "FN";
        case TK_RET:
            return "RET";
        case TK_IF:
            return "IF";
        case TK_ELIF:
            return "ELIF";
        case TK_ELSE:
            return "ELSE";
        case TK_LPAREN:
            return "LPAREN";
        case TK_RPAREN:
            return "RPAREN";
        case TK_LBRACE:
            return "LBRACE";
        case TK_RBRACE:
            return "RBRACE";
        case TK_LOGOR:
            return "LOGOR";
        case TK_LOGAND:
            return "LOGAND";
        case TK_EQ:
            return "EQ";
        case TK_NEQ:
            return "NEQ";
        case TK_GT:
            return "GT";
        case TK_LT:
            return "LT";
        case TK_GE:
            return "GE";
        case TK_LE:
            return "LE";
        case TK_ADD:
            return "ADD";
        case TK_MIN:
            return "MIN";
        case TK_MUL:
            return "MUL";
        case TK_DIV:
            return "DIV";
        case TK_MOD:
            return "MOD";
        case TK_NOT:
            return "NOT";
        case TK_PWR:
            return "PWR";
        case TK_COMMA:
            return "COMMA";
        case TK_FLOAT:
            return "FLOAT";
        case TK_INT:
            return "INT";
        case TK_BOOL:
            return "BOOL";
        case TK_EOS:
            return "EOS";
        case TK_IDENT:
            return "IDENT";
        case TK_LET:
            return "LET";
        case TK_ASSIGN:
            return "ASSIGN";
        case TK_EOF:
            return "EOF";
    }
}

struct Token {
    TokenType tk;
    Value val;
};

struct Tokenizer {
    size_t pos;
    std::string src;
    std::vector<Token> toks;

    bool at_end() {
        return pos >= src.size();
    }

    char peek() {
        if (at_end()) {
            return '\0';
        }
        return src[pos];
    }

    char peek(int n) {
        if (pos + n >= src.size()) {
            return '\0';
        }
        return src[pos + n];
    }

    char advance() {
        if (at_end()) {
            return '\0';
        }
        return src[pos++];
    }

    char advance(int n) {
        if (at_end()) {
            return '\0';
        }

        char c = src[pos];
        pos += n;
        return c;
    }

    void skip_ws() {
        while (isspace(peek())) {
            advance();
        }
    }

    void push(Token tk) {
        toks.push_back(tk);
    }

    bool starts_with(std::string_view s) const {
        return src.compare(pos, s.size(), s) == 0;
    }

    std::string parse_idname() {
        skip_ws();
        if (!isalpha(peek()) && peek() != '_') {
            throw std::runtime_error("expected identifier");
        }
        std::string name;
        while (isalnum(peek()) || peek() == '_') {
            name.push_back(advance());
        }
        return name;
    }

    bool is_ident() {
        return (isalpha(peek()) || peek() == '_') && !is_keyword();
    }

    bool is_keyword() {
        return starts_with("fn") ||
               starts_with("ret") ||
               starts_with("if") ||
               starts_with("elif") ||
               starts_with("else") ||
               starts_with("let") ||
               starts_with("true") ||
               starts_with("false");
    }
};

std::vector<Token> tokenize(std::string src);