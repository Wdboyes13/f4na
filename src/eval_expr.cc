#include <eval.h>
#include <stdexcept>

Value ExprEval::operator()(BinaryExpr& e) {
    auto lv = eval_expr(env, e.left);
    auto rv = eval_expr(env, e.right);

    delete e.left;
    delete e.right;

    switch (e.op) {
        case TK_LOGOR: {
            return lv || rv;
        }
        case TK_LOGAND: {
            return lv && rv;
        }
        case TK_EQ: {
            return lv == rv;
        }
        case TK_NEQ: {
            return lv != rv;
        }
        case TK_GT: {
            return lv > rv;
        }
        case TK_LT: {
            return lv < rv;
        }
        case TK_GE: {
            return lv >= rv;
        }
        case TK_LE: {
            return lv <= rv;
        }
        case TK_MIN: {
            return lv - rv;
        }
        case TK_ADD: {
            return lv + rv;
        }
        case TK_MUL: {
            return lv * rv;
        }
        case TK_DIV: {
            return lv / rv;
        }
        case TK_MOD: {
            return lv % rv;
        }
        case TK_PWR: {
            return lv.pow(rv);
        }
        case TK_BITOR: {
            return lv | rv;
        }
        case TK_BITAND: {
            return lv & rv;
        }
        case TK_BITXOR: {
            return lv ^ rv;
        }
        case TK_SHL: {
            return lv << rv;
        }
        case TK_SHR: {
            return lv >> rv;
        }

            BADCASE
    }
}

Value ExprEval::operator()(UnaryExpr& e) {
    Value val = eval_expr(env, e.arg);
    delete e.arg;

    switch (e.op) {
        case TK_NOT: {
            return !val;
        }
        case TK_ADD: {
            return +val;
        }
        case TK_MIN: {
            return -val;
        }
        case TK_BNOT: {
            return ~val;
        }

            BADCASE
    }
}

Value ExprEval::operator()(CallExpr& e) {
    std::vector<Value> args;
    for (auto& arg : e.args) {
        args.push_back(eval_expr(env, arg));
        delete arg;
    }

    if (env->bfnlut.contains(e.ident)) {
        auto fn = env->bfnlut[e.ident];
        if (fn.nargs == args.size() || fn.nargs == -1) {
            return fn.fn(args);
        } else {
            throw std::runtime_error(
                "incorrect number of arguments for function \"" +
                std::to_string(fn.nargs) +
                "\" got " +
                std::to_string(args.size()));
        }
    } else if (env->ufnlut.contains(e.ident)) {
        auto fn = env->ufnlut[e.ident];
        if (fn.args.size() != args.size()) {
            throw std::runtime_error(
                "incorrect number of arguments for function \"" +
                std::to_string(fn.args.size()) +
                "\" got " +
                std::to_string(args.size()));
        }

        auto svars = env->vars;
        env->vars = fn.vars;

        for (int i = 0; i < args.size(); i++) {
            env->vars[fn.args[i]] = args[i];
        }

        try {
            eval_block(env, fn.body);
        } catch (ReturnException& ret) {
            env->vars = svars;
            return ret.get_value();
        }
    }

    throw std::runtime_error("unknown function");
}

Value ExprEval::operator()(IdentExpr& e) {
    if (env->vars.contains(e.ident)) {
        return env->vars[e.ident];
    } else {
        throw std::runtime_error("unknown ident");
    }
}

Value ExprEval::operator()(LiteralExpr& e) {
    return e.val;
}

Value eval_expr(Environment* env, Expr* expr) {
    return std::visit(ExprEval{ env }, *expr);
}