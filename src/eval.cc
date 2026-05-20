#include <eval.h>
#include <stdexcept>
#include "ast.h"

void StmtEval::operator()(LetStmt& e) {
    auto val = eval_expr(env, e.expr);
    delete e.expr;

    env->vars.insert({ e.name, val });
}

void StmtEval::operator()(AssignStmt& e) {
    auto val = eval_expr(env, e.expr);
    delete e.expr;

    if (env->vars.contains(e.name)) {
        env->vars[e.name] = val;
    } else {
        throw std::runtime_error("unknown ident");
    }
}

void StmtEval::operator()(RetStmt& e) {
    auto val = eval_expr(env, e.expr);
    delete e.expr;

    throw ReturnException(val);
}

void StmtEval::operator()(IfStmt& e) {
    bool ran = false;
    for (auto& branch : e.branches) {
        if (eval_expr(env, branch.cond).as_int()) {
            ran = true;
            eval_block(env, branch.body);
            break;
        }
    }

    if (e.else_body.has_value() && !ran) {
        eval_block(env, e.else_body.value());
    }
}

void StmtEval::operator()(FnDeclStmt& e) {
    env->ufnlut[e.name] = { e.args, env->vars, e.body };
}

void StmtEval::operator()(ExprStmt& e) {
    eval_expr(env, e.expr);
    delete e.expr;
}

void StmtEval::operator()(WhileStmt& e) {
    while (eval_expr(env, e.blk.cond).as_int()) {
        eval_block(env, e.blk.body);
    }
}

void eval_stmt(Environment* env, Stmt& stmt) {
    std::visit(
        StmtEval{ env },
        stmt);
}

void eval_block(Environment* env, Block& blk) {
    for (auto& stmt : blk.stmts) {
        eval_stmt(env, stmt);
    }
}

void eval(std::vector<Stmt> prog) {
    auto env = build_env();
    for (auto& stmt : prog) {
        eval_stmt(env, stmt);
    }
}