#pragma once
#include <ast.h>

struct BuiltinFn;
struct UserFn;

typedef std::unordered_map<std::string, BuiltinFn> BuiltinLUT;
typedef std::unordered_map<std::string, UserFn> UserFnLUT;
typedef std::unordered_map<std::string, Value> VarLUT;

class ReturnException {
  public:
    ReturnException(Value v) : v(v) {}
    Value get_value() { return v; }

  private:
    Value v;
};

struct Environment {
    VarLUT vars;
    BuiltinLUT bfnlut;
    UserFnLUT ufnlut;
};

struct BuiltinFn {
    int nargs;
    std::function<Value(std::vector<Value>)> fn;
};

struct UserFn {
    std::vector<std::string> args;
    VarLUT vars;
    Block body;
};

Environment* build_env();

struct StmtEval {
    Environment* env;
    void operator()(LetStmt& e);
    void operator()(AssignStmt& e);
    void operator()(RetStmt& e);
    void operator()(IfStmt& e);
    void operator()(FnDeclStmt& e);
    void operator()(ExprStmt& e);
    void operator()(WhileStmt& e);
};

struct ExprEval {
    Environment* env;
    Value operator()(BinaryExpr& e);
    Value operator()(UnaryExpr& e);
    Value operator()(CallExpr& e);
    Value operator()(IdentExpr& e);
    Value operator()(LiteralExpr& e);
};

#define BADCASE \
    default:    \
        throw std::runtime_error("invalid expr");

Value eval_expr(Environment* env, Expr* expr);
void eval_block(Environment* env, Block& blk);
void eval_stmt(Environment* env, Stmt& stmt);
void eval(std::vector<Stmt> prog);