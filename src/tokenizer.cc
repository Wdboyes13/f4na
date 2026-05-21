#include <tokenizer.h>
#include <cctype>
#include <fstream>
#include <stdexcept>

bool has_import(std::vector<std::string> imported, std::string import_path) {
    for (const auto& imp : imported) {
        if (imp == import_path) {
            return true;
        }
    }
    return false;
}

std::vector<Token> tokenize(std::string src) {
    Tokenizer tk{};
    tk.src = src;

    std::vector<std::string> imported = {};

    while (!tk.at_end()) {
        tk.skip_ws();
        if (tk.starts_with("import")) {
            size_t stmt_start = tk.pos;
            tk.advance(6);
            tk.skip_ws();
            std::string import_path;
            if (tk.peek() == '"') {
                tk.advance();
                std::string pbuf;
                while (tk.peek() != '"') {
                    pbuf.push_back(tk.advance());
                }
                tk.advance();
            } else if (tk.is_ident()) {
                import_path = tk.parse_idname() + ".f4na";
            } else {
                throw std::runtime_error("expected path or identifier for import");
            }

            if (has_import(imported, import_path)) {
                if (tk.peek() != ';') {
                    throw std::runtime_error("expected EOS after import");
                }
                tk.advance();
                tk.src.erase(stmt_start, tk.pos - stmt_start);
                continue;
            }

            std::ifstream file(import_path);
            if (!file) {
                throw std::runtime_error("failed to open file: " + import_path);
            }

            std::string import_src{
                std::istreambuf_iterator<char>(file),
                std::istreambuf_iterator<char>()
            };

            file.close();

            if (tk.peek() != ';') {
                throw std::runtime_error("expected EOS after import");
            }
            tk.advance();
            tk.src.replace(stmt_start, tk.pos - stmt_start, import_src + "\n");
            imported.push_back(import_path);
            tk.pos = stmt_start;
        } else {
            tk.advance();
        }
    }

    tk.pos = 0;

    while (!tk.at_end()) {
        tk.skip_ws();
        auto cr = tk.peek();
        if (cr == '"') {
            tk.advance();
            std::string buf;
            while (tk.peek() != '"' && !tk.at_end()) {
                buf.push_back(tk.advance());
            }
            tk.advance();
            tk.push({ TK_STRING, buf });
        } else if (tk.is_keyword()) {
            if (tk.starts_with("fn")) {
                tk.advance(2);
                tk.push({ TK_FN });
            } else if (tk.starts_with("ret")) {
                tk.advance(3);
                tk.push({ TK_RET });
            } else if (tk.starts_with("if")) {
                tk.advance(2);
                tk.push({ TK_IF });
            } else if (tk.starts_with("elif")) {
                tk.advance(4);
                tk.push({ TK_ELIF });
            } else if (tk.starts_with("else")) {
                tk.advance(4);
                tk.push({ TK_ELSE });
            } else if (tk.starts_with("let")) {
                tk.advance(3);
                tk.push({ TK_LET });
            } else if (tk.starts_with("true")) {
                tk.advance(4);
                tk.push({ TK_BOOL, true });
            } else if (tk.starts_with("false")) {
                tk.advance(5);
                tk.push({ TK_BOOL, false });
            }
        } else if (cr == '(') {
            tk.advance();
            tk.push({ TK_LPAREN });
        } else if (cr == ')') {
            tk.advance();
            tk.push({ TK_RPAREN });
        } else if (cr == '{') {
            tk.advance();
            tk.push({ TK_LBRACE });
        } else if (cr == '}') {
            tk.advance();
            tk.push({ TK_RBRACE });
        } else if (tk.starts_with("||")) {
            tk.advance(2);
            tk.push({ TK_LOGOR });
        } else if (tk.starts_with("&&")) {
            tk.advance(2);
            tk.push({ TK_LOGAND });
        } else if (tk.starts_with("==")) {
            tk.advance(2);
            tk.push({ TK_EQ });
        } else if (tk.starts_with("!=")) {
            tk.advance(2);
            tk.push({ TK_NEQ });
        } else if (cr == '|') {
            tk.advance();
            tk.push({ TK_BITOR });
        } else if (cr == '&') {
            tk.advance();
            tk.push({ TK_BITAND });
        } else if (cr == '^') {
            tk.advance();
            tk.push({ TK_BITXOR });
        } else if (cr == '>') {
            tk.advance();
            if (tk.peek() == '=') {
                tk.advance();
                tk.push({ TK_GE });
            } else if (tk.peek() == '>') {
                tk.advance();
                tk.push({ TK_SHR });
            } else {
                tk.push({ TK_GT });
            }
        } else if (cr == '<') {
            tk.advance();
            if (tk.peek() == '=') {
                tk.advance();
                tk.push({ TK_LE });
            } else if (tk.peek() == '<') {
                tk.advance();
                tk.push({ TK_SHL });
            } else {
                tk.push({ TK_LT });
            }
        } else if (cr == '+') {
            tk.advance();
            tk.push({ TK_ADD });
        } else if (cr == '-') {
            tk.advance();
            tk.push({ TK_MIN });
        } else if (cr == '*') {
            tk.advance();
            if (tk.peek() == '*') {
                tk.advance();
                tk.push({ TK_PWR });
            } else {
                tk.push({ TK_MUL });
            }
        } else if (cr == '/') {
            tk.advance();
            tk.push({ TK_DIV });
        } else if (cr == '%') {
            tk.advance();
            tk.push({ TK_MOD });
        } else if (cr == '!') {
            tk.advance();
            tk.push({ TK_NOT });
        } else if (cr == '~') {
            tk.advance();
            tk.push({ TK_BNOT });
        } else if (cr == ',') {
            tk.advance();
            tk.push({ TK_COMMA });
        } else if (cr == '=') {
            tk.advance();
            tk.push({ TK_ASSIGN });
        } else if (tk.is_ident()) {
            auto ident = tk.parse_idname();
            tk.push({ TK_IDENT, ident });
        } else if (cr == ';') {
            tk.advance();
            tk.push({ TK_EOS });
        } else {
            std::string buf = {};
            enum { HEX,
                   NORM,
                   OCT,
                   BIN
            } ntype = NORM;
            if (tk.starts_with("0x")) {
                ntype = HEX;
                tk.advance(2);
            } else if (tk.starts_with("0o")) {
                ntype = OCT;
                tk.advance(2);
            } else if (tk.starts_with("0b")) {
                ntype = BIN;
                tk.advance(2);
            }

            auto isok = [&](char c) -> bool {
                if (ntype == NORM && isdigit(c)) {
                    return true;
                } else if (ntype == HEX && isxdigit(c)) {
                    return true;
                } else if (ntype == OCT && c >= '0' && c <= '7') {
                    return true;
                } else if (ntype == BIN && (c == '0' || c == '1')) {
                    return true;
                } else {
                    return false;
                }
            };

            while (isok(tk.peek())) {
                buf.push_back(tk.advance());
            }

            if (ntype == NORM && tk.peek() == '.') {
                buf.push_back(tk.advance());
                while (isdigit(tk.peek())) {
                    buf.push_back(tk.advance());
                }
                tk.push({ TK_FLOAT, std::stod(buf) });
            } else {
                int base = (ntype == HEX)   ? 16
                           : (ntype == OCT) ? 8
                           : (ntype == BIN) ? 2
                                            : 10;
                tk.push({ TK_INT, std::stoll(buf) });
            }
        }
    }

    tk.push({ TK_EOF });

    return tk.toks;
}